// AUTOGENERATED FROM index-windows-1252.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1252.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338,
    141, 381, 143, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482,
    353, 8250, 339, 157, 382, 376, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183,
    184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198,
    199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213,
    214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228,
    229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243,
    244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        8364 => 128, 129 => 129, 8218 => 130, 402 => 131, 8222 => 132,
        8230 => 133, 8224 => 134, 8225 => 135, 710 => 136, 8240 => 137,
        352 => 138, 8249 => 139, 338 => 140, 141 => 141, 381 => 142,
        143 => 143, 144 => 144, 8216 => 145, 8217 => 146, 8220 => 147,
        8221 => 148, 8226 => 149, 8211 => 150, 8212 => 151, 732 => 152,
        8482 => 153, 353 => 154, 8250 => 155, 339 => 156, 157 => 157,
        382 => 158, 376 => 159, 160 => 160, 161 => 161, 162 => 162, 163 => 163,
        164 => 164, 165 => 165, 166 => 166, 167 => 167, 168 => 168, 169 => 169,
        170 => 170, 171 => 171, 172 => 172, 173 => 173, 174 => 174, 175 => 175,
        176 => 176, 177 => 177, 178 => 178, 179 => 179, 180 => 180, 181 => 181,
        182 => 182, 183 => 183, 184 => 184, 185 => 185, 186 => 186, 187 => 187,
        188 => 188, 189 => 189, 190 => 190, 191 => 191, 192 => 192, 193 => 193,
        194 => 194, 195 => 195, 196 => 196, 197 => 197, 198 => 198, 199 => 199,
        200 => 200, 201 => 201, 202 => 202, 203 => 203, 204 => 204, 205 => 205,
        206 => 206, 207 => 207, 208 => 208, 209 => 209, 210 => 210, 211 => 211,
        212 => 212, 213 => 213, 214 => 214, 215 => 215, 216 => 216, 217 => 217,
        218 => 218, 219 => 219, 220 => 220, 221 => 221, 222 => 222, 223 => 223,
        224 => 224, 225 => 225, 226 => 226, 227 => 227, 228 => 228, 229 => 229,
        230 => 230, 231 => 231, 232 => 232, 233 => 233, 234 => 234, 235 => 235,
        236 => 236, 237 => 237, 238 => 238, 239 => 239, 240 => 240, 241 => 241,
        242 => 242, 243 => 243, 244 => 244, 245 => 245, 246 => 246, 247 => 247,
        248 => 248, 249 => 249, 250 => 250, 251 => 251, 252 => 252, 253 => 253,
        254 => 254, 255 => 255, _ => 0
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
