// AUTOGENERATED FROM index-gb18030.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-gb18030.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u32] = &[
    0, 128, 165, 169, 178, 184, 216, 226, 235, 238, 244, 248, 251, 253, 258,
    276, 284, 300, 325, 329, 334, 364, 463, 465, 467, 469, 471, 473, 475, 477,
    506, 594, 610, 712, 716, 730, 930, 938, 962, 970, 1026, 1104, 1106, 8209,
    8215, 8218, 8222, 8231, 8241, 8244, 8246, 8252, 8365, 8452, 8454, 8458,
    8471, 8482, 8556, 8570, 8596, 8602, 8713, 8720, 8722, 8726, 8731, 8737,
    8740, 8742, 8748, 8751, 8760, 8766, 8777, 8781, 8787, 8802, 8808, 8816,
    8854, 8858, 8870, 8896, 8979, 9322, 9372, 9548, 9588, 9616, 9622, 9634,
    9652, 9662, 9672, 9676, 9680, 9702, 9735, 9738, 9793, 9795, 11906, 11909,
    11913, 11917, 11928, 11944, 11947, 11951, 11956, 11960, 11964, 11979,
    12284, 12292, 12312, 12319, 12330, 12351, 12436, 12447, 12535, 12543,
    12586, 12842, 12850, 12964, 13200, 13215, 13218, 13253, 13263, 13267,
    13270, 13384, 13428, 13727, 13839, 13851, 14617, 14703, 14801, 14816,
    14964, 15183, 15471, 15585, 16471, 16736, 17208, 17325, 17330, 17374,
    17623, 17997, 18018, 18212, 18218, 18301, 18318, 18760, 18811, 18814,
    18820, 18823, 18844, 18848, 18872, 19576, 19620, 19738, 19887, 40870,
    59244, 59336, 59367, 59413, 59417, 59423, 59431, 59437, 59443, 59452,
    59460, 59478, 59493, 63789, 63866, 63894, 63976, 63986, 64016, 64018,
    64021, 64025, 64034, 64037, 64042, 65074, 65093, 65107, 65112, 65127,
    65132, 65375, 65510, 65536,
];

static BACKWARD_TABLE: &'static [u32] = &[
    0, 0, 36, 38, 45, 50, 81, 89, 95, 96, 100, 103, 104, 105, 109, 126, 133,
    148, 172, 175, 179, 208, 306, 307, 308, 309, 310, 311, 312, 313, 341, 428,
    443, 544, 545, 558, 741, 742, 749, 750, 805, 819, 820, 7922, 7924, 7925,
    7927, 7934, 7943, 7944, 7945, 7950, 8062, 8148, 8149, 8152, 8164, 8174,
    8236, 8240, 8262, 8264, 8374, 8380, 8381, 8384, 8388, 8390, 8392, 8393,
    8394, 8396, 8401, 8406, 8416, 8419, 8424, 8437, 8439, 8445, 8482, 8485,
    8496, 8521, 8603, 8936, 8946, 9046, 9050, 9063, 9066, 9076, 9092, 9100,
    9108, 9111, 9113, 9131, 9162, 9164, 9218, 9219, 11329, 11331, 11334, 11336,
    11346, 11361, 11363, 11366, 11370, 11372, 11375, 11389, 11682, 11686,
    11687, 11692, 11694, 11714, 11716, 11723, 11725, 11730, 11736, 11982,
    11989, 12102, 12336, 12348, 12350, 12384, 12393, 12395, 12397, 12510,
    12553, 12851, 12962, 12973, 13738, 13823, 13919, 13933, 14080, 14298,
    14585, 14698, 15583, 15847, 16318, 16434, 16438, 16481, 16729, 17102,
    17122, 17315, 17320, 17402, 17418, 17859, 17909, 17911, 17915, 17916,
    17936, 17939, 17961, 18664, 18703, 18814, 18962, 19043, 33469, 33470,
    33471, 33484, 33485, 33490, 33497, 33501, 33505, 33513, 33520, 33536,
    33550, 37845, 37921, 37948, 38029, 38038, 38064, 38065, 38066, 38069,
    38075, 38076, 38078, 39108, 39109, 39113, 39114, 39115, 39116, 39265,
    39394, 189000,
];

#[inline]
pub fn forward(code: u32) -> u32 {
    if (code > 39419 && code < 189000) || code > 1237575 { return 0xffffffff; }
    let mut i = if code >= BACKWARD_TABLE[127] {81} else {0};
    if code >= BACKWARD_TABLE[i+63] { i += 64; }
    if code >= BACKWARD_TABLE[i+31] { i += 32; }
    if code >= BACKWARD_TABLE[i+15] { i += 16; }
    if code >= BACKWARD_TABLE[i+7] { i += 8; }
    if code >= BACKWARD_TABLE[i+3] { i += 4; }
    if code >= BACKWARD_TABLE[i+1] { i += 2; }
    if code >= BACKWARD_TABLE[i] { i += 1; }
    (code - BACKWARD_TABLE[i-1]) + FORWARD_TABLE[i-1]
}

#[inline]
pub fn backward(code: u32) -> u32 {
    if code < 128 { return 0xffffffff; }
    let mut i = if code >= FORWARD_TABLE[127] {81} else {0};
    if code >= FORWARD_TABLE[i+63] { i += 64; }
    if code >= FORWARD_TABLE[i+31] { i += 32; }
    if code >= FORWARD_TABLE[i+15] { i += 16; }
    if code >= FORWARD_TABLE[i+7] { i += 8; }
    if code >= FORWARD_TABLE[i+3] { i += 4; }
    if code >= FORWARD_TABLE[i+1] { i += 2; }
    if code >= FORWARD_TABLE[i] { i += 1; }
    (code - FORWARD_TABLE[i-1]) + BACKWARD_TABLE[i-1]
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_no_failure() {
        for i in range(0u32, 189002) {
            forward(i);
        }
        for j in range(127u32, 65538) {
            backward(j);
        }
    }

    #[test]
    fn test_correct_table() {
        for i in range(0u32, 189002) {
            let j = forward(i);
            if j == 0xffffffff { continue; }
            let i_ = backward(j);
            if i_ == 0xffffffff { continue; }
            assert!(i_ == i, "backward(forward({})) = backward({}) = {} != {}", i, j, i_, i);
        }
    }
}
