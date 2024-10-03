#![no_main]

use sp1_zkvm::syscalls::syscall_bls12381_add;

sp1_zkvm::entrypoint!(main);

// generator.
// 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
// 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
const A: [u8; 96] = [
    187, 198, 34, 219, 10, 240, 58, 251, 239, 26, 122, 249, 63, 232, 85, 108, 88, 172, 27, 23, 63,
    58, 78, 161, 5, 185, 116, 151, 79, 140, 104, 195, 15, 172, 169, 79, 140, 99, 149, 38, 148, 215,
    151, 49, 167, 211, 241, 23, 225, 231, 197, 70, 41, 35, 170, 12, 228, 138, 136, 162, 68, 199,
    60, 208, 237, 179, 4, 44, 203, 24, 219, 0, 246, 10, 208, 213, 149, 224, 245, 252, 228, 138, 29,
    116, 237, 48, 158, 160, 241, 160, 170, 227, 129, 244, 179, 8,
];

// 2 * generator.
// 838589206289216005799424730305866328161735431124665289961769162861615689790485775997575391185127590486775437397838
// 3450209970729243429733164009999191867485184320918914219895632678707687208996709678363578245114137957452475385814312
const B: [u8; 96] = [
    78, 15, 191, 41, 85, 140, 154, 195, 66, 124, 28, 143, 187, 117, 143, 226, 42, 166, 88, 195, 10,
    45, 144, 67, 37, 1, 40, 145, 48, 219, 33, 151, 12, 69, 169, 80, 235, 200, 8, 136, 70, 103, 77,
    144, 234, 203, 114, 5, 40, 157, 116, 121, 25, 136, 134, 186, 27, 189, 22, 205, 212, 217, 86,
    76, 106, 215, 95, 29, 2, 185, 59, 247, 97, 228, 112, 134, 203, 62, 186, 34, 56, 142, 157, 119,
    115, 166, 253, 34, 163, 115, 198, 171, 140, 157, 106, 22,
];

// 3 * generator.
// 1527649530533633684281386512094328299672026648504329745640827351945739272160755686119065091946435084697047221031460
// 487897572011753812113448064805964756454529228648704488481988876974355015977479905373670519228592356747638779818193
const C: [u8; 96] = [
    36, 82, 78, 2, 201, 192, 210, 150, 155, 23, 162, 44, 11, 122, 116, 129, 249, 63, 91, 51, 81,
    10, 120, 243, 241, 165, 233, 155, 31, 214, 18, 177, 151, 150, 169, 236, 45, 33, 101, 23, 19,
    240, 209, 249, 8, 227, 236, 9, 209, 48, 174, 144, 5, 59, 71, 163, 92, 244, 74, 99, 108, 37, 69,
    231, 230, 59, 212, 15, 49, 39, 156, 157, 127, 9, 195, 171, 221, 12, 154, 166, 12, 248, 197,
    137, 51, 98, 132, 138, 159, 176, 245, 166, 211, 128, 43, 3,
];

pub fn main() {
<<<<<<< HEAD
    common_test_utils::weierstrass_add::test_weierstrass_add::<
        Bls12381AffinePoint,
        { sp1_lib::bls12381::N },
    >(&A, &B, &C, sp1_curves::weierstrass::bls12_381::Bls12381BaseField::MODULUS);
=======
    for _ in 0..4 {
        // generator.
        // 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
        // 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
        let mut a: [u8; 96] = [
            187, 198, 34, 219, 10, 240, 58, 251, 239, 26, 122, 249, 63, 232, 85, 108, 88, 172, 27,
            23, 63, 58, 78, 161, 5, 185, 116, 151, 79, 140, 104, 195, 15, 172, 169, 79, 140, 99,
            149, 38, 148, 215, 151, 49, 167, 211, 241, 23, 225, 231, 197, 70, 41, 35, 170, 12, 228,
            138, 136, 162, 68, 199, 60, 208, 237, 179, 4, 44, 203, 24, 219, 0, 246, 10, 208, 213,
            149, 224, 245, 252, 228, 138, 29, 116, 237, 48, 158, 160, 241, 160, 170, 227, 129, 244,
            179, 8,
        ];

        // 2 * generator.
        // 838589206289216005799424730305866328161735431124665289961769162861615689790485775997575391185127590486775437397838
        // 3450209970729243429733164009999191867485184320918914219895632678707687208996709678363578245114137957452475385814312
        let b: [u8; 96] = [
            78, 15, 191, 41, 85, 140, 154, 195, 66, 124, 28, 143, 187, 117, 143, 226, 42, 166, 88,
            195, 10, 45, 144, 67, 37, 1, 40, 145, 48, 219, 33, 151, 12, 69, 169, 80, 235, 200, 8,
            136, 70, 103, 77, 144, 234, 203, 114, 5, 40, 157, 116, 121, 25, 136, 134, 186, 27, 189,
            22, 205, 212, 217, 86, 76, 106, 215, 95, 29, 2, 185, 59, 247, 97, 228, 112, 134, 203,
            62, 186, 34, 56, 142, 157, 119, 115, 166, 253, 34, 163, 115, 198, 171, 140, 157, 106,
            22,
        ];

        syscall_bls12381_add(a.as_mut_ptr() as *mut [u32; 24], b.as_ptr() as *const [u32; 24]);

        // 3 * generator.
        // 1527649530533633684281386512094328299672026648504329745640827351945739272160755686119065091946435084697047221031460
        // 487897572011753812113448064805964756454529228648704488481988876974355015977479905373670519228592356747638779818193
        let c: [u8; 96] = [
            36, 82, 78, 2, 201, 192, 210, 150, 155, 23, 162, 44, 11, 122, 116, 129, 249, 63, 91,
            51, 81, 10, 120, 243, 241, 165, 233, 155, 31, 214, 18, 177, 151, 150, 169, 236, 45, 33,
            101, 23, 19, 240, 209, 249, 8, 227, 236, 9, 209, 48, 174, 144, 5, 59, 71, 163, 92, 244,
            74, 99, 108, 37, 69, 231, 230, 59, 212, 15, 49, 39, 156, 157, 127, 9, 195, 171, 221,
            12, 154, 166, 12, 248, 197, 137, 51, 98, 132, 138, 159, 176, 245, 166, 211, 128, 43, 3,
        ];

        assert_eq!(a, c);
    }

    println!("done");
>>>>>>> origin/dev
}
