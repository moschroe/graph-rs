use rust_onedrive::drive::endpoint::DriveEndPoint;

#[test]
fn drive_endpoint_v1_test() {
    assert_eq!(
        DriveEndPoint::Drive.v1_url(),
        "https://graph.microsoft.com/v1.0/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveMe.v1_url(),
        "https://graph.microsoft.com/v1.0/me/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveRoot.v1_url(),
        "https://graph.microsoft.com/v1.0/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootMe.v1_url(),
        "https://graph.microsoft.com/v1.0/me/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootChild.v1_url(),
        "https://graph.microsoft.com/v1.0/drive/root/children"
    );
    assert_eq!(
        DriveEndPoint::SharedWithMe.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe"
    );
    assert_eq!(
        DriveEndPoint::DriveRecent.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/recent"
    );
    assert_eq!(
        DriveEndPoint::DriveActivitiesMe.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/activities"
    );
    assert_eq!(
        DriveEndPoint::SpecialFolder.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special"
    );
    assert_eq!(
        DriveEndPoint::SpecialDocuments.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents"
    );
    assert_eq!(
        DriveEndPoint::SpecialDocumentsChild.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialPhotos.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/photos"
    );
    assert_eq!(
        DriveEndPoint::SpecialPhotosChild.to_string(),
        "https://graph.microsoft.com/v1.0/me/special/photos/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialCameraRoll.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll"
    );
    assert_eq!(
        DriveEndPoint::SpecialCameraRollChild.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialAppRoot.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot"
    );
    assert_eq!(
        DriveEndPoint::SpecialAppRootChild.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialMusic.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/music"
    );
    assert_eq!(
        DriveEndPoint::SpecialMusicChild.to_string(),
        "https://graph.microsoft.com/v1.0/me/drive/special/music/children"
    );
}

#[test]
fn drive_endpoint_beta_test() {
    assert_eq!(
        DriveEndPoint::Drive.beta_url(),
        "https://graph.microsoft.com/beta/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveMe.beta_url(),
        "https://graph.microsoft.com/beta/me/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveRoot.beta_url(),
        "https://graph.microsoft.com/beta/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootMe.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootChild.beta_url(),
        "https://graph.microsoft.com/beta/drive/root/children"
    );
    assert_eq!(
        DriveEndPoint::SharedWithMe.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/sharedWithMe"
    );
    assert_eq!(
        DriveEndPoint::DriveRecent.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/recent"
    );
    assert_eq!(
        DriveEndPoint::DriveActivitiesMe.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/activities"
    );
    assert_eq!(
        DriveEndPoint::SpecialFolder.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special"
    );
    assert_eq!(
        DriveEndPoint::SpecialDocuments.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/documents"
    );
    assert_eq!(
        DriveEndPoint::SpecialDocumentsChild.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/documents/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialPhotos.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/photos"
    );
    assert_eq!(
        DriveEndPoint::SpecialPhotosChild.beta_url(),
        "https://graph.microsoft.com/beta/me/special/photos/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialCameraRoll.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/cameraroll"
    );
    assert_eq!(
        DriveEndPoint::SpecialCameraRollChild.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/cameraroll/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialAppRoot.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/approot"
    );
    assert_eq!(
        DriveEndPoint::SpecialAppRootChild.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/approot/children"
    );
    assert_eq!(
        DriveEndPoint::SpecialMusic.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/music"
    );
    assert_eq!(
        DriveEndPoint::SpecialMusicChild.beta_url(),
        "https://graph.microsoft.com/beta/me/drive/special/music/children"
    );
}
