// primitive types can be converted to each other through casting.
// rust addresses conversion between custom types (ie. struct and enum)
// by the use of `traits`. The generic conversion will use the `From` and
// `Into` traits. However there are more specific ones for the more common
// cases, in particular when converting to and from `String s`
pub mod fa_from_and_into;
pub mod fb_tryfrom_and_tryinto;
pub mod fc_to_and_from_strings;