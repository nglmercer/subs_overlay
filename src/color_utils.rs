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
            eprintln!(
                "Invalid color format: {}. Expected #RRGGBB or #AARRGGBB",
                color
            );
            "#000000".to_string()
        }
    }
}

/// Valida formato de color
#[allow(dead_code)]
pub fn is_valid_color(color: &str) -> bool {
    if !color.starts_with('#') && !color.starts_with("0x") {
        return false;
    }
    let hex = color.trim_start_matches('#').trim_start_matches("0x");
    matches!(hex.len(), 3 | 4 | 6 | 8)
}

/// Convierte string hex a u32 ARGB
/// Soporta formatos: #RGB, #ARGB, #RRGGBB, #AARRGGBB
/// También soporta prefijo 0x
#[allow(dead_code)]
pub fn hex_to_argb_u32(color: &str) -> u32 {
    let hex = color.trim_start_matches('#').trim_start_matches("0x");

    match hex.len() {
        3 => {
            // #RGB -> #FFRRGGBB
            let r = u32::from_str_radix(&hex[0..1], 16).unwrap_or(0xF);
            let g = u32::from_str_radix(&hex[1..2], 16).unwrap_or(0xF);
            let b = u32::from_str_radix(&hex[2..3], 16).unwrap_or(0xF);
            0xFF000000 | (r * 17) << 16 | (g * 17) << 8 | (b * 17)
        }
        4 => {
            // #ARGB -> #AARRGGBB
            let a = u32::from_str_radix(&hex[0..1], 16).unwrap_or(0xF);
            let r = u32::from_str_radix(&hex[1..2], 16).unwrap_or(0xF);
            let g = u32::from_str_radix(&hex[2..3], 16).unwrap_or(0xF);
            let b = u32::from_str_radix(&hex[3..4], 16).unwrap_or(0xF);
            (a * 17) << 24 | (r * 17) << 16 | (g * 17) << 8 | (b * 17)
        }
        6 => {
            // #RRGGBB -> #FFRRGGBB
            let rgb = u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFF);
            0xFF000000 | rgb
        }
        8 => {
            // #AARRGGBB
            u32::from_str_radix(hex, 16).unwrap_or(0xFFFFFFFF)
        }
        _ => {
            eprintln!(
                "Invalid color format: {}. Defaulting to opaque white.",
                color
            );
            0xFFFFFFFF
        }
    }
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
        assert!(is_valid_color("#F00"));
        assert!(is_valid_color("#F00F"));
        assert!(!is_valid_color("FF0000")); // Missing #
        assert!(is_valid_color("#FF00")); // Now supported (4 digits)
        assert!(!is_valid_color("#FF0000000"));
    }

    #[test]
    fn test_hex_to_argb_u32() {
        // 6 digits
        assert_eq!(hex_to_argb_u32("#FFFFFF"), 0xFFFFFFFF);
        assert_eq!(hex_to_argb_u32("#000000"), 0xFF000000);
        assert_eq!(hex_to_argb_u32("#FF0000"), 0xFFFF0000);

        // 8 digits
        assert_eq!(hex_to_argb_u32("#FFFFFFFF"), 0xFFFFFFFF);
        assert_eq!(hex_to_argb_u32("#00FFFFFF"), 0x00FFFFFF); // Transparent white
        assert_eq!(hex_to_argb_u32("#80000000"), 0x80000000); // Semi-transparent black

        // 3 digits
        assert_eq!(hex_to_argb_u32("#FFF"), 0xFFFFFFFF);
        assert_eq!(hex_to_argb_u32("#F00"), 0xFFFF0000);
        assert_eq!(hex_to_argb_u32("#0F0"), 0xFF00FF00);
        assert_eq!(hex_to_argb_u32("#00F"), 0xFF0000FF);

        // 4 digits
        assert_eq!(hex_to_argb_u32("#FFFF"), 0xFFFFFFFF);
        assert_eq!(hex_to_argb_u32("#0FFF"), 0x00FFFFFF);
        assert_eq!(hex_to_argb_u32("#F000"), 0xFF000000);

        // Invalid
        assert_eq!(hex_to_argb_u32("invalid"), 0xFFFFFFFF);
    }
}
