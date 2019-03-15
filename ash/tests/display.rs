extern crate ash;
use ash::vk;

#[test]
fn display_flags() {
    assert_eq!(
        (vk::AccessFlags::INDIRECT_COMMAND_READ | vk::AccessFlags::VERTEX_ATTRIBUTE_READ)
            .to_string(),
        "INDIRECT_COMMAND_READ | VERTEX_ATTRIBUTE_READ"
    );
}

#[test]
fn display_enum() {
    assert_eq!(vk::ChromaLocation::MIDPOINT.to_string(), "MIDPOINT");
}

#[test]
fn display_result() {
    assert_eq!(vk::Result::SUCCESS.to_string(), "SUCCESS");
}

#[test]
fn error_result_description() {
    use std::error::Error;
    assert_eq!(
        vk::Result::SUCCESS.description(),
        "Command completed successfully"
    );
}
