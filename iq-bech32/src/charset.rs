/// Default separator character
pub const SEPARATOR: char = '1';

/// bech32 encoding character set
pub const CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];

/// Inverse mapping from character codes to CHARSET indexes
pub const CHARSET_INVERSE: [Option<u8>; 128] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(15),
    None,
    Some(10),
    Some(17),
    Some(21),
    Some(20),
    Some(26),
    Some(30),
    Some(7),
    Some(5),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(29),
    None,
    Some(24),
    Some(13),
    Some(25),
    Some(9),
    Some(8),
    Some(23),
    None,
    Some(18),
    Some(22),
    Some(31),
    Some(27),
    Some(19),
    None,
    Some(1),
    Some(0),
    Some(3),
    Some(16),
    Some(11),
    Some(28),
    Some(12),
    Some(14),
    Some(6),
    Some(4),
    Some(2),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(29),
    None,
    Some(24),
    Some(13),
    Some(25),
    Some(9),
    Some(8),
    Some(23),
    None,
    Some(18),
    Some(22),
    Some(31),
    Some(27),
    Some(19),
    None,
    Some(1),
    Some(0),
    Some(3),
    Some(16),
    Some(11),
    Some(28),
    Some(12),
    Some(14),
    Some(6),
    Some(4),
    Some(2),
    None,
    None,
    None,
    None,
    None,
];
