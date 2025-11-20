/// Convierte color #AARRGGBB o #RRGGBB a #RRGGBB
/// Slint NO acepta alpha en formato hex
#[allow(dead_code)]
pub fn to_slint_color_string(color: &str) -> String {
    let hex = color.trim_start_matches('#');

    match hex.len() {
        8 => {
            // Asumimos #AARRGGBB (alpha primero)
            format!("#{}", &hex[2..8])
        }
        6 => {
            // Ya es #RRGGBB
            format!("#{}", hex)
        }
        _ => {
            eprintln!("Invalid color format: {}. Expected #RRGGBB or #AARRGGBB", color);
            "#000000".to_string()
        }
    }
}

/// Valida formato de color
#[allow(dead_code)]
pub fn is_valid_color(color: &str) -> bool {
    let hex = color.trim_start_matches('#');
    hex.len() == 6 || hex.len() == 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        assert_eq!(to_slint_color_string("#CC000000"), "#000000");
        assert_eq!(to_slint_color_string("#FFFFFFFF"), "#FFFFFF");
        assert_eq!(to_slint_color_string("#FFFFFF00"), "#FFFF00");
        assert_eq!(to_slint_color_string("#FF0000"), "#FF0000");
    }

    #[test]
    fn test_color_validation() {
        assert!(is_valid_color("#FF0000"));
        assert!(is_valid_color("#CCFF0000"));
        assert!(!is_valid_color("FF0000"));
        assert!(!is_valid_color("#FF00"));
        assert!(!is_valid_color("#FF000000"));
    }
}
