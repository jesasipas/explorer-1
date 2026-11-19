//! Fonctions pures de la maquette (docs/maquette/README.md) : aucune dépendance egui.

/// Code couleur d'un niveau d'alerte, indépendant du moteur de rendu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    Nominal,
    Warning,
    Danger,
}

const BATTERY_EMPTY_V: f32 = 6.4;
const BATTERY_FULL_V: f32 = 8.4;

/// Pourcentage de charge batterie à partir de la tension (LiPo 2S), borné à [0, 100].
pub fn battery_percent(voltage: f32) -> f32 {
    let ratio = (voltage - BATTERY_EMPTY_V) / (BATTERY_FULL_V - BATTERY_EMPTY_V);
    ratio.clamp(0.0, 1.0) * 100.0
}

/// Niveau d'alerte de la batterie : ambre à 40 %, rouge à 20 %.
pub fn battery_alert_level(percent: f32) -> AlertLevel {
    if percent <= 20.0 {
        AlertLevel::Danger
    } else if percent <= 40.0 {
        AlertLevel::Warning
    } else {
        AlertLevel::Nominal
    }
}

/// Nombre de barres réseau (0 à 4) à partir de la latence en millisecondes.
pub fn network_bars(latency_ms: f32) -> u8 {
    if latency_ms < 20.0 {
        4
    } else if latency_ms < 50.0 {
        3
    } else if latency_ms < 100.0 {
        2
    } else if latency_ms < 200.0 {
        1
    } else {
        0
    }
}

/// Conversion polaire -> cartésien (repère robot : x vers l'avant, y vers la gauche).
///
/// Une distance non finie (`NaN`, infinie) ou négative n'a pas de position valide : elle
/// renvoie `None` plutôt que de produire des coordonnées non finies (voir
/// docs/troubleshooting.md, leçon transposée depuis les plantages Qt sur données capteur).
pub fn polar_to_cartesian(angle_rad: f32, distance_m: f32) -> Option<(f32, f32)> {
    if !distance_m.is_finite() || distance_m < 0.0 {
        return None;
    }
    Some((distance_m * angle_rad.cos(), distance_m * angle_rad.sin()))
}

/// Couleur d'alerte d'un point lidar selon sa distance. `None` (donnée invalide) est traité
/// comme nominal : on ne colore pas en danger une absence de mesure.
pub fn lidar_point_alert_level(distance_m: f32) -> AlertLevel {
    if !distance_m.is_finite() {
        return AlertLevel::Nominal;
    }
    if distance_m < 0.5 {
        AlertLevel::Danger
    } else if distance_m < 1.5 {
        AlertLevel::Warning
    } else {
        AlertLevel::Nominal
    }
}

/// Vitesses maximales (linéaire m/s, angulaire rad/s) pour un niveau de vitesse 1 à 5.
///
/// Un niveau hors de [1, 5] est ramené dans l'intervalle plutôt que de paniquer : la source
/// du niveau (manette, interface) peut transmettre une valeur hors bornes transitoire.
pub fn speed_limits_for_level(level: u8) -> (f32, f32) {
    let level = level.clamp(1, 5) as f32;
    (level * 0.06, level * 0.2)
}

/// Portées lidar sélectionnables (mètres), dans l'ordre de L1/R1 de la maquette.
pub const LIDAR_RANGES_M: [f32; 5] = [1.5, 3.0, 5.0, 8.0, 12.0];

/// Portée lidar pour un index de palier (ramené dans les bornes de `LIDAR_RANGES_M`).
pub fn lidar_range_for_index(index: usize) -> f32 {
    LIDAR_RANGES_M[index.min(LIDAR_RANGES_M.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_percent_bounds() {
        assert_eq!(battery_percent(6.4), 0.0);
        assert_eq!(battery_percent(8.4), 100.0);
        assert!((battery_percent(7.4) - 50.0).abs() < 1e-3);
    }

    #[test]
    fn battery_percent_clamps_out_of_range_voltage() {
        assert_eq!(battery_percent(5.0), 0.0);
        assert_eq!(battery_percent(9.5), 100.0);
    }

    #[test]
    fn battery_alert_thresholds() {
        assert_eq!(battery_alert_level(41.0), AlertLevel::Nominal);
        assert_eq!(battery_alert_level(40.0), AlertLevel::Warning);
        assert_eq!(battery_alert_level(21.0), AlertLevel::Warning);
        assert_eq!(battery_alert_level(20.0), AlertLevel::Danger);
        assert_eq!(battery_alert_level(0.0), AlertLevel::Danger);
    }

    #[test]
    fn network_bars_thresholds() {
        assert_eq!(network_bars(0.0), 4);
        assert_eq!(network_bars(19.9), 4);
        assert_eq!(network_bars(20.0), 3);
        assert_eq!(network_bars(49.9), 3);
        assert_eq!(network_bars(50.0), 2);
        assert_eq!(network_bars(99.9), 2);
        assert_eq!(network_bars(100.0), 1);
        assert_eq!(network_bars(199.9), 1);
        assert_eq!(network_bars(200.0), 0);
        assert_eq!(network_bars(5000.0), 0);
    }

    #[test]
    fn polar_to_cartesian_basic() {
        let (x, y) = polar_to_cartesian(0.0, 2.0).unwrap();
        assert!((x - 2.0).abs() < 1e-6);
        assert!(y.abs() < 1e-6);

        let (x, y) = polar_to_cartesian(std::f32::consts::FRAC_PI_2, 1.0).unwrap();
        assert!(x.abs() < 1e-6);
        assert!((y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn polar_to_cartesian_rejects_invalid_distance() {
        assert_eq!(polar_to_cartesian(0.0, f32::NAN), None);
        assert_eq!(polar_to_cartesian(0.0, f32::INFINITY), None);
        assert_eq!(polar_to_cartesian(0.0, -1.0), None);
    }

    #[test]
    fn lidar_point_alert_thresholds() {
        assert_eq!(lidar_point_alert_level(0.49), AlertLevel::Danger);
        assert_eq!(lidar_point_alert_level(0.5), AlertLevel::Warning);
        assert_eq!(lidar_point_alert_level(1.49), AlertLevel::Warning);
        assert_eq!(lidar_point_alert_level(1.5), AlertLevel::Nominal);
        assert_eq!(lidar_point_alert_level(100.0), AlertLevel::Nominal);
    }

    #[test]
    fn lidar_point_alert_treats_invalid_as_nominal() {
        assert_eq!(lidar_point_alert_level(f32::NAN), AlertLevel::Nominal);
        assert_eq!(lidar_point_alert_level(f32::INFINITY), AlertLevel::Nominal);
    }

    #[test]
    fn speed_limits_scale_with_level() {
        let (linear, angular) = speed_limits_for_level(1);
        assert!((linear - 0.06).abs() < 1e-6);
        assert!((angular - 0.2).abs() < 1e-6);

        let (linear, angular) = speed_limits_for_level(5);
        assert!((linear - 0.30).abs() < 1e-6);
        assert!((angular - 1.0).abs() < 1e-6);
    }

    #[test]
    fn speed_limits_clamp_out_of_range_level() {
        assert_eq!(speed_limits_for_level(0), speed_limits_for_level(1));
        assert_eq!(speed_limits_for_level(9), speed_limits_for_level(5));
    }

    #[test]
    fn lidar_range_for_index_clamps() {
        assert_eq!(lidar_range_for_index(0), 1.5);
        assert_eq!(lidar_range_for_index(4), 12.0);
        assert_eq!(lidar_range_for_index(99), 12.0);
    }
}
