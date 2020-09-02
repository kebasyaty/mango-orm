//! # Field Types
//!
//! Field Types for Models.

// FIELD TYPES =====================================================================================
/// Field Types for Models
/// CheckBox
#[derive(Debug, Clone)]
pub enum CheckBox {
    Data(Option<bool>),
}
impl Default for CheckBox {
    fn default() -> Self {
        CheckBox::Data(Option::None)
    }
}
impl CheckBox {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Color
#[derive(Debug, Clone)]
pub enum Color {
    Data(Option<&'static str>),
}
impl Default for Color {
    fn default() -> Self {
        Color::Data(Option::None)
    }
}
impl Color {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Date
#[derive(Debug, Clone)]
pub enum Date {
    Data(Option<&'static str>),
}
impl Default for Date {
    fn default() -> Self {
        Date::Data(Option::None)
    }
}
impl Date {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Email
#[derive(Debug, Clone)]
pub enum Email {
    Data(Option<&'static str>),
}
impl Default for Email {
    fn default() -> Self {
        Email::Data(Option::None)
    }
}
impl Email {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// File
#[derive(Debug, Clone)]
pub enum File {
    Data(Option<&'static str>),
}
impl Default for File {
    fn default() -> Self {
        File::Data(Option::None)
    }
}
impl File {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Image
#[derive(Debug, Clone)]
pub enum Image {
    Data(Option<&'static str>),
}
impl Default for Image {
    fn default() -> Self {
        Image::Data(Option::None)
    }
}
impl Image {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberI64
#[derive(Debug, Clone)]
pub enum NumberI64 {
    Data(Option<i64>),
}
impl Default for NumberI64 {
    fn default() -> Self {
        NumberI64::Data(Option::None)
    }
}
impl NumberI64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberU64
#[derive(Debug, Clone)]
pub enum NumberU64 {
    Data(Option<i64>),
}
impl Default for NumberU64 {
    fn default() -> Self {
        NumberU64::Data(Option::None)
    }
}
impl NumberU64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberF64
#[derive(Debug, Clone)]
pub enum NumberF64 {
    Data(Option<f64>),
}
impl Default for NumberF64 {
    fn default() -> Self {
        NumberF64::Data(Option::None)
    }
}
impl NumberF64 {
    pub fn get_data(&self) -> Option<f64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Password
#[derive(Debug, Clone)]
pub enum Password {
    Data(Option<&'static str>),
}
impl Default for Password {
    fn default() -> Self {
        Password::Data(Option::None)
    }
}
impl Password {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Radio
#[derive(Debug, Clone)]
pub enum Radio {
    Data(Option<bool>),
}
impl Default for Radio {
    fn default() -> Self {
        Radio::Data(Option::None)
    }
}
impl Radio {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Tel
#[derive(Debug, Clone)]
pub enum Tel {
    Data(Option<&'static str>),
}
impl Default for Tel {
    fn default() -> Self {
        Tel::Data(Option::None)
    }
}
impl Tel {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// TextLine
#[derive(Debug, Clone)]
pub enum TextLine {
    Data(Option<&'static str>),
}
impl Default for TextLine {
    fn default() -> Self {
        TextLine::Data(Option::None)
    }
}
impl TextLine {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Time
#[derive(Debug, Clone)]
pub enum Time {
    Data(Option<&'static str>),
}
impl Default for Time {
    fn default() -> Self {
        Time::Data(Option::None)
    }
}
impl Time {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Url
#[derive(Debug, Clone)]
pub enum Url {
    Data(Option<&'static str>),
}
impl Default for Url {
    fn default() -> Self {
        Url::Data(Option::None)
    }
}
impl Url {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// TextArea
#[derive(Debug, Clone)]
pub enum TextArea {
    Data(Option<&'static str>),
}
impl Default for TextArea {
    fn default() -> Self {
        TextArea::Data(Option::None)
    }
}
impl TextArea {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectText
#[derive(Debug, Clone)]
pub enum SelectText {
    Data(Option<&'static str>),
}
impl Default for SelectText {
    fn default() -> Self {
        SelectText::Data(Option::None)
    }
}
impl SelectText {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectI64
#[derive(Debug, Clone)]
pub enum SelectI64 {
    Data(Option<i64>),
}
impl Default for SelectI64 {
    fn default() -> Self {
        SelectI64::Data(Option::None)
    }
}
impl SelectI64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectU64
#[derive(Debug, Clone)]
pub enum SelectU64 {
    Data(Option<u64>),
}
impl Default for SelectU64 {
    fn default() -> Self {
        SelectU64::Data(Option::None)
    }
}
impl SelectU64 {
    pub fn get_data(&self) -> Option<u64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectF64
#[derive(Debug, Clone)]
pub enum SelectF64 {
    Data(Option<f64>),
}
impl Default for SelectF64 {
    fn default() -> Self {
        SelectF64::Data(Option::None)
    }
}
impl SelectF64 {
    pub fn get_data(&self) -> Option<f64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// ForeignKey
#[derive(Debug, Clone)]
pub enum ForeignKey {
    Data(Option<&'static str>),
}
impl Default for ForeignKey {
    fn default() -> Self {
        ForeignKey::Data(Option::None)
    }
}
impl ForeignKey {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// ManyToMany
#[derive(Debug, Clone)]
pub enum ManyToMany {
    Data(Option<&'static str>),
}
impl Default for ManyToMany {
    fn default() -> Self {
        ManyToMany::Data(Option::None)
    }
}
impl ManyToMany {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// OneToOne
#[derive(Debug, Clone)]
pub enum OneToOne {
    Data(Option<&'static str>),
}
impl Default for OneToOne {
    fn default() -> Self {
        OneToOne::Data(Option::None)
    }
}
impl OneToOne {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}