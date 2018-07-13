#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    RoomArchived,
    RoomCreated,
    RoomDeleted,
    RoomEnter,
    RoomExit,
    RoomFileUpload,
    RoomMessage,
    RoomNotification,
    RoomTopicChange,
    RoomUnarchived,
}
