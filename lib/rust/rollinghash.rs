pub mod mylib {
    /// A rolling hash implementation for ASCII strings.
    ///
    /// This implementation uses a modulo of 2^61 - 1 to reduce hash collisions.
    pub struct RollingHash {
        hash: Vec<u64>,
        power: Vec<u64>,
        length: usize,
    }

    impl RollingHash {
        const MOD: u64 = (1 << 61) - 1;
        const MASK30: u64 = (1 << 30) - 1;
        const MASK31: u64 = (1 << 31) - 1;
        const MASK61: u64 = Self::MOD;
        const POSITIVISER: u64 = Self::MOD * 4;

        /// Multiplies two numbers in a way that avoids overflow under modulo 2^61 - 1 arithmetic.
        ///
        /// This function splits the input values into high and low parts,
        /// multiplies the parts separately, and then recombines them.
        ///
        /// **Note:** The result is not reduced modulo 2^61 - 1. To obtain the final value
        /// within the correct range, apply `calc_mod` on the result.
        fn mul(a: u64, b: u64) -> u64 {
            let au = a >> 31;
            let ad = a & Self::MASK31;
            let bu = b >> 31;
            let bd = b & Self::MASK31;
            let mid = ad * bu + au * bd;
            let midu = mid >> 30;
            let midd = mid & Self::MASK30;

            au * bu * 2 + midu + (midd << 31) + ad * bd
        }

        /// Reduces the given value modulo 2^61 - 1.
        ///
        /// It splits the value into its upper and lower parts relative to 2^61 and
        /// then combines them to ensure the result is within the modulo.
        fn calc_mod(x: u64) -> u64 {
            let xu = x >> 61;
            let xd = x & Self::MASK61;
            let ret = xu + xd;

            if ret >= Self::MOD {
                ret - Self::MOD
            } else {
                ret
            }
        }

        /// Creates a new `RollingHash` for the given ASCII string.
        ///
        /// The hash and power values for each prefix of the input are precomputed.
        ///
        /// # Arguments
        ///
        /// * `s` - A slice of `char` representing an ASCII string.
        /// * `base` - The base used for the polynomial rolling hash.
        pub fn new(s: &[char], base: u64) -> Self {
            let length = s.len();
            let mut hash = vec![0; length + 1];
            let mut power = vec![1; length + 1];

            for (i, &ch) in s.iter().enumerate() {
                let c = ch as u64;
                hash[i + 1] = Self::calc_mod(Self::mul(hash[i], base) + c);
                power[i + 1] = Self::calc_mod(Self::mul(power[i], base));
            }

            Self {
                hash,
                power,
                length,
            }
        }

        /// Returns the hash value for the substring `s[l..r]`.
        ///
        /// The hash of the substring is computed in O(1) time using the precomputed values.
        ///
        /// # Arguments
        ///
        /// * `l` - The starting index (inclusive).
        /// * `r` - The ending index (exclusive).
        ///
        /// # Returns
        ///
        /// The hash value of the substring.
        pub fn get_hash(&self, l: usize, r: usize) -> u64 {
            Self::calc_mod(
                self.hash[r] + Self::POSITIVISER - Self::mul(self.hash[l], self.power[r - l]),
            )
        }

        /// Returns the length of the input string.
        pub fn len(&self) -> usize {
            self.length
        }

        /// Returns `true` if the input string is empty.
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    }
}
