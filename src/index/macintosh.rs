// AUTOGENERATED FROM index-macintosh.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-macintosh.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233,
    232, 234, 235, 237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249,
    251, 252, 8224, 176, 162, 163, 167, 8226, 182, 223, 174, 169, 8482, 180,
    168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181, 8706, 8721, 8719,
    960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216,
    8217, 247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183,
    8218, 8222, 8240, 194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212,
    63743, 210, 218, 219, 217, 305, 710, 732, 175, 728, 729, 730, 184, 733,
    731, 711,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        196 => 128, 197 => 129, 199 => 130, 201 => 131, 209 => 132, 214 => 133,
        220 => 134, 225 => 135, 224 => 136, 226 => 137, 228 => 138, 227 => 139,
        229 => 140, 231 => 141, 233 => 142, 232 => 143, 234 => 144, 235 => 145,
        237 => 146, 236 => 147, 238 => 148, 239 => 149, 241 => 150, 243 => 151,
        242 => 152, 244 => 153, 246 => 154, 245 => 155, 250 => 156, 249 => 157,
        251 => 158, 252 => 159, 8224 => 160, 176 => 161, 162 => 162,
        163 => 163, 167 => 164, 8226 => 165, 182 => 166, 223 => 167,
        174 => 168, 169 => 169, 8482 => 170, 180 => 171, 168 => 172,
        8800 => 173, 198 => 174, 216 => 175, 8734 => 176, 177 => 177,
        8804 => 178, 8805 => 179, 165 => 180, 181 => 181, 8706 => 182,
        8721 => 183, 8719 => 184, 960 => 185, 8747 => 186, 170 => 187,
        186 => 188, 937 => 189, 230 => 190, 248 => 191, 191 => 192, 161 => 193,
        172 => 194, 8730 => 195, 402 => 196, 8776 => 197, 8710 => 198,
        171 => 199, 187 => 200, 8230 => 201, 160 => 202, 192 => 203,
        195 => 204, 213 => 205, 338 => 206, 339 => 207, 8211 => 208,
        8212 => 209, 8220 => 210, 8221 => 211, 8216 => 212, 8217 => 213,
        247 => 214, 9674 => 215, 255 => 216, 376 => 217, 8260 => 218,
        8364 => 219, 8249 => 220, 8250 => 221, 64257 => 222, 64258 => 223,
        8225 => 224, 183 => 225, 8218 => 226, 8222 => 227, 8240 => 228,
        194 => 229, 202 => 230, 193 => 231, 203 => 232, 200 => 233, 205 => 234,
        206 => 235, 207 => 236, 204 => 237, 211 => 238, 212 => 239,
        63743 => 240, 210 => 241, 218 => 242, 219 => 243, 217 => 244,
        305 => 245, 710 => 246, 732 => 247, 175 => 248, 728 => 249, 729 => 250,
        730 => 251, 184 => 252, 733 => 253, 731 => 254, 711 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
