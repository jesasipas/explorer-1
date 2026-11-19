//! Contrat de données entre une source de télémétrie (démo ou future passerelle réseau)
//! et l'interface. Aucun type ici ne dépend d'egui ni de ROS 2 : c'est la frontière.

/// État d'une connexion logique (robot ou manette).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

/// Qualité réseau mesurée, ou liaison perdue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkQuality {
    /// Latence aller-retour en millisecondes.
    LatencyMs(f32),
    Disconnected,
}

/// Tension batterie brute en volts (LiPo 2S : 6.4V = 0 %, 8.4V = 100 %, voir `logic`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryVoltage(pub f32);

/// Un point de balayage lidar en coordonnées polaires, tel que livré par le capteur.
///
/// `distance_m` peut être `NaN` ou infini : un point lidar invalide (hors de portée, écho
/// perdu) doit être représentable sans paniquer. Voir `logic::lidar_point_color` et
/// `logic::polar_to_cartesian` pour la gestion de ces valeurs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LidarPoint {
    pub angle_rad: f32,
    pub distance_m: f32,
}

/// Un balayage lidar complet, décrit par ses bornes angulaires plutôt qu'un angle par point.
#[derive(Debug, Clone, PartialEq)]
pub struct LidarScan {
    pub angle_min_rad: f32,
    pub angle_increment_rad: f32,
    /// Distances en mètres, dans l'ordre des angles croissants depuis `angle_min_rad`.
    /// Une valeur `NaN` ou infinie signale un point invalide.
    pub distances_m: Vec<f32>,
}

impl LidarScan {
    /// Reconstruit les points polaires du balayage.
    pub fn points(&self) -> impl Iterator<Item = LidarPoint> + '_ {
        self.distances_m
            .iter()
            .enumerate()
            .map(|(i, &distance_m)| LidarPoint {
                angle_rad: self.angle_min_rad + self.angle_increment_rad * i as f32,
                distance_m,
            })
    }
}

/// Image vidéo décodée, prête à être affichée.
///
/// [TODO] Le format exact dépendra de la caméra retenue au jalon M1
/// (voir `docs/architecture.md`, section Topics). On choisit ici RGB8 packé, le format le
/// plus simple à convertir en `egui::ColorImage`, en supposant qu'une passerelle réseau fera
/// le décodage (JPEG/H.264 -> RGB8) avant de pousser la trame dans ce contrat : l'interface
/// ne doit pas dépendre d'un codec vidéo particulier.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    /// Pixels RGB8, taille attendue : `width * height * 3`.
    pub rgb8: Vec<u8>,
}

/// Télémétrie complète poussée par une source vers l'interface.
#[derive(Debug, Clone, PartialEq)]
pub struct Telemetry {
    pub robot_connection: ConnectionState,
    pub controller_connection: ConnectionState,
    pub network: NetworkQuality,
    pub battery: BatteryVoltage,
    pub lidar: LidarScan,
    pub emergency_stop_engaged: bool,
}

/// Consignes de déplacement, dans le repère du robot.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MotionCommand {
    pub vx_mps: f32,
    pub vy_mps: f32,
    pub wz_radps: f32,
}

/// Source de télémétrie : pousse données et images vers l'interface.
///
/// Une implémentation tourne dans son propre thread (voir `demo::DemoSource`) et ne doit
/// jamais bloquer le thread de la boucle egui : la réception se fait par canal (`try_recv`
/// côté interface), jamais par appel direct depuis le thread UI.
pub trait TelemetrySource {
    fn try_recv_telemetry(&mut self) -> Option<Telemetry>;
    fn try_recv_video_frame(&mut self) -> Option<VideoFrame>;
}

/// Puits de commandes : reçoit les décisions prises dans l'interface (ou depuis une
/// manette, branchée plus tard via `gilrs` dans une PR suivante).
pub trait CommandSink {
    fn set_emergency_stop(&mut self, engaged: bool);
    /// Niveau de vitesse, de 1 à 5 (voir `logic::speed_limits_for_level`).
    fn set_speed_level(&mut self, level: u8);
    fn send_motion_command(&mut self, command: MotionCommand);
}
