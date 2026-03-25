#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPrimalCharacterStatusComponent {
    #[offset(0x00C8)]
    pub MaxStatusValues: MaxStatusValues,

    #[offset(0x0708)]
    pub BaseCharacterLevel:  i32,
    pub ExtraCharacterLevel: u16,

    #[offset(0x0858)]
    pub CurrentStatusValues: MaxStatusValues,
}

#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct MaxStatusValues {
    pub health:    f32,
    pub stamina:   f32,
    pub torpidity: f32,
    pub oxygen:    f32,
    pub food:      f32,
    pub water:     f32,
    _padding_7:    f32,
    pub weight:    f32,
    _padding_8:    f32,
    _padding_9:    f32,
    pub fortitude: f32,
    _padding_11:   f32,
}
