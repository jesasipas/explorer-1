//! Source de démonstration : anime les 4-5 états de la maquette (docs/maquette/README.md)
//! dans un thread dédié, poussés par canal. Sélectionnée via `--demo`.

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use crate::contract::{
    BatteryVoltage, ConnectionState, LidarScan, NetworkQuality, Telemetry, TelemetrySource,
    VideoFrame,
};

const TICK: Duration = Duration::from_millis(50);
const LIDAR_POINTS: usize = 360;

/// Les scènes rejouées en boucle par la démo, dans l'ordre de la maquette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scene {
    Moving,
    Stopped,
    EmergencyStop,
    LowBattery,
    LinkLost,
}

impl Scene {
    const SEQUENCE: [(Scene, u32); 5] = [
        (Scene::Moving, 100),
        (Scene::Stopped, 40),
        (Scene::EmergencyStop, 40),
        (Scene::LowBattery, 60),
        (Scene::LinkLost, 40),
    ];

    fn at_tick(tick: u64) -> Scene {
        let total: u32 = Self::SEQUENCE.iter().map(|(_, n)| n).sum();
        let mut remainder = (tick % total as u64) as u32;
        for (scene, duration) in Self::SEQUENCE {
            if remainder < duration {
                return scene;
            }
            remainder -= duration;
        }
        unreachable!("la somme des durées couvre tout le cycle")
    }
}

fn lidar_scan_for_scene(scene: Scene, tick: u64) -> LidarScan {
    let sweep = (tick as f32 * 0.05) % std::f32::consts::TAU;
    let base_distance = match scene {
        Scene::LinkLost => f32::NAN,
        Scene::EmergencyStop | Scene::Stopped | Scene::LowBattery => 2.0,
        Scene::Moving => 1.2,
    };
    let distances_m = (0..LIDAR_POINTS)
        .map(|i| {
            if base_distance.is_nan() {
                return f32::NAN;
            }
            let angle = i as f32 * std::f32::consts::TAU / LIDAR_POINTS as f32;
            // Un obstacle animé qui s'approche dans le sens du balayage, pour donner du
            // mouvement à la mini-carte de démonstration.
            let proximity = ((angle - sweep).cos() + 1.0) * 0.5;
            base_distance - proximity * 0.8
        })
        .collect();
    LidarScan {
        angle_min_rad: 0.0,
        angle_increment_rad: std::f32::consts::TAU / LIDAR_POINTS as f32,
        distances_m,
    }
}

fn telemetry_for_scene(scene: Scene, tick: u64) -> Telemetry {
    let lidar = lidar_scan_for_scene(scene, tick);
    match scene {
        Scene::Moving => Telemetry {
            robot_connection: ConnectionState::Connected,
            controller_connection: ConnectionState::Connected,
            network: NetworkQuality::LatencyMs(15.0),
            battery: BatteryVoltage(7.8),
            lidar,
            emergency_stop_engaged: false,
        },
        Scene::Stopped => Telemetry {
            robot_connection: ConnectionState::Connected,
            controller_connection: ConnectionState::Connected,
            network: NetworkQuality::LatencyMs(25.0),
            battery: BatteryVoltage(7.6),
            lidar,
            emergency_stop_engaged: false,
        },
        Scene::EmergencyStop => Telemetry {
            robot_connection: ConnectionState::Connected,
            controller_connection: ConnectionState::Connected,
            network: NetworkQuality::LatencyMs(18.0),
            battery: BatteryVoltage(7.5),
            lidar,
            emergency_stop_engaged: true,
        },
        Scene::LowBattery => Telemetry {
            robot_connection: ConnectionState::Connected,
            controller_connection: ConnectionState::Connected,
            network: NetworkQuality::LatencyMs(30.0),
            battery: BatteryVoltage(6.9),
            lidar,
            emergency_stop_engaged: false,
        },
        Scene::LinkLost => Telemetry {
            robot_connection: ConnectionState::Disconnected,
            controller_connection: ConnectionState::Connected,
            network: NetworkQuality::Disconnected,
            battery: BatteryVoltage(7.5),
            lidar,
            emergency_stop_engaged: false,
        },
    }
}

/// Image vidéo de démonstration : un décor dessiné, comme indiqué dans la maquette.
///
/// [TODO] Dégradé uni en remplacement d'un vrai décor dessiné : suffisant pour exercer le
/// chemin de rendu vidéo sans dépendre d'un asset binaire dans ce dépôt.
fn video_frame_for_scene(scene: Scene, tick: u64) -> VideoFrame {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 200;
    let shift = (tick % WIDTH as u64) as u8;
    let (base_r, base_g, base_b): (u8, u8, u8) = match scene {
        Scene::LinkLost => (20, 20, 24),
        Scene::EmergencyStop => (40, 10, 10),
        _ => (10, 14, 20),
    };
    let mut rgb8 = Vec::with_capacity((WIDTH * HEIGHT * 3) as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            rgb8.push(base_r.wrapping_add(((x as u8).wrapping_add(shift)) / 4));
            rgb8.push(base_g.wrapping_add((y as u8) / 4));
            rgb8.push(base_b);
        }
    }
    VideoFrame {
        width: WIDTH,
        height: HEIGHT,
        rgb8,
    }
}

/// Source animée tournant dans son propre thread ; ne bloque jamais le thread appelant.
pub struct DemoSource {
    telemetry_rx: Receiver<Telemetry>,
    video_rx: Receiver<VideoFrame>,
    _handle: thread::JoinHandle<()>,
}

impl DemoSource {
    pub fn spawn() -> Self {
        let (telemetry_tx, telemetry_rx) = channel();
        let (video_tx, video_rx) = channel();
        let handle = thread::spawn(move || run(telemetry_tx, video_tx));
        Self {
            telemetry_rx,
            video_rx,
            _handle: handle,
        }
    }
}

fn run(telemetry_tx: Sender<Telemetry>, video_tx: Sender<VideoFrame>) {
    let mut tick: u64 = 0;
    loop {
        let scene = Scene::at_tick(tick);
        if telemetry_tx.send(telemetry_for_scene(scene, tick)).is_err() {
            return;
        }
        if video_tx.send(video_frame_for_scene(scene, tick)).is_err() {
            return;
        }
        tick += 1;
        thread::sleep(TICK);
    }
}

impl TelemetrySource for DemoSource {
    fn try_recv_telemetry(&mut self) -> Option<Telemetry> {
        match self.telemetry_rx.try_recv() {
            Ok(telemetry) => Some(telemetry),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }

    fn try_recv_video_frame(&mut self) -> Option<VideoFrame> {
        match self.video_rx.try_recv() {
            Ok(frame) => Some(frame),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_sequence_covers_full_cycle_without_gaps() {
        let total: u32 = Scene::SEQUENCE.iter().map(|(_, n)| n).sum();
        for tick in 0..(total as u64 * 2) {
            // Ne doit jamais paniquer sur `unreachable!` quel que soit le tick.
            let _ = Scene::at_tick(tick);
        }
    }

    #[test]
    fn link_lost_scene_has_invalid_lidar_distances() {
        let scan = lidar_scan_for_scene(Scene::LinkLost, 0);
        assert!(scan.distances_m.iter().all(|d| d.is_nan()));
    }
}
