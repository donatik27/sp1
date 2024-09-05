#![no_main]

use sp1_zkvm::syscalls::syscall_ed_add;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    for _ in 0..4 {
        // 90393249858788985237231628593243673548167146579814268721945474994541877372611
        // 33321104029277118100578831462130550309254424135206412570121538923759338004303
        let mut a: [u32; 16] = [
            3483215555, 3316636890, 393982319, 1735542857, 3477272092, 3938161218, 2441473672,
            3352871983, 518004559, 1412659891, 3171159156, 381601478, 4186949959, 2241017948,
            562215772, 1235948440,
        ];

        // 61717728572175158701898635111983295176935961585742968051419350619945173564869
        // 28137966556353620208933066709998005335145594788896528644015312259959272398451
        let b: [u32; 16] = [
            1305001413, 1765397705, 2860221887, 645018279, 498886181, 2392712151, 1132535804,
            2289237784, 928931443, 3765815339, 770636683, 3578174032, 1825132251, 759745353,
            801604519, 1043695186,
        ];

        syscall_ed_add(&mut a, &b);

        // 36213413123116753589144482590359479011148956763279542162278577842046663495729
        // 17093345531692682197799066694073110060588941459686871373458223451938707761683
        let c: [u32; 16] = [
            3313602609, 809411414, 3369455838, 916675291, 3769825491, 3689654254, 4022204411,
            1343230146, 3323226643, 1869035592, 1797114180, 3905015883, 380615349, 2113780693,
            3521007660, 634027423,
        ];

        assert_eq!(a, c);
    }

    println!("done");
}
