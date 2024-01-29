use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AppointmentFlags: u32 {
        /// The value `Other`, at bit position `0`.
        const OTHER = 0b00000001;
        /// The value `Wax`, at bit position `1`.
        const WAX = 0b00000010;
        /// The value `Edge`, at bit position `2`.
        const EDGE = 0b00000100;


        /// The combination of `A`, `B`, and `C`.
        const FULLTUNE = Self::WAX.bits() | Self::EDGE.bits();
    }
}

// example from docs:

// fn main() {
//     let e1 = Flags::A | Flags::C;
//     let e2 = Flags::B | Flags::C;
//     assert_eq!((e1 | e2), Flags::ABC);   // union
//     assert_eq!((e1 & e2), Flags::C);     // intersection
//     assert_eq!((e1 - e2), Flags::A);     // set difference
//     assert_eq!(!e2, Flags::A);           // set complement
// }
