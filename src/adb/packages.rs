extern crate adb_client;

pub mod debloat {
    use adb_client::AdbTcpConnection;
    use std::net::Ipv4Addr;

    pub fn uninstall() {
        let vectors: Vec<Vec<&str>> = vec![
            // {{{ FACEBOOK
            // vec!["pm", "uninstall", "--user", "0", "com.facebook.services"],
            // vec!["pm", "uninstall", "--user", "0", "com.facebook.appmanager"],
            // vec!["pm", "uninstall", "--user", "0", "com.facebook.system"],
            // vec!["pm", "uninstall", "--user", "0", "com.facebook.katana"],
            // }}}

            // {{{ GOOGLE
            // AR
            vec!["pm", "uninstall", "--user", "0", "com.google.ar.core"],
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.mimage.avatarstickers"],
            // Assistant
            vec!["pm", "uninstall", "--user", "0", "com.google.android.apps.googleassistant"],
            // Bookmarksprovider
            vec!["pm", "uninstall", "--user", "0", "com.android.bookmarkprovider"],
            // Chrome
            vec!["pm", "uninstall", "--user", "0", "com.android.chrome"],
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.app.chromecustomizations"],
            // Feedback
            vec!["pm", "uninstall", "--user", "0", "com.google.android.feedback"],
            // Google app
            vec!["pm", "uninstall", "--user", "0", "com.google.android.googlequicksearchbox"],
            // Health
            vec!["pm", "uninstall", "--user", "0", "com.google.android.healthconnect.controller"],
            vec!["pm", "uninstall", "--user", "0", "com.google.android.health.connect.backuprestore"],
            // Meet
            vec!["pm", "uninstall", "--user", "0", "com.google.android.apps.tachyon"],
            // Messaging
            vec!["pm", "uninstall", "--user", "0", "com.google.android.apps.messaging"],
            // Screensaver
            vec!["pm", "uninstall", "--user", "0", "com.android.dreams.basic"],
            vec!["pm", "uninstall", "--user", "0", "com.android.dreams.phototable"],
            // }}}

            // {{{ MICROSOFT
            vec!["pm", "uninstall", "--user", "0", "com.microsoft.appmanager"],
            vec!["pm", "uninstall", "--user", "0", "com.microsoft.skydrive"],
            vec!["pm", "uninstall", "--user", "0", "com.touchtype.swiftkey"],
            vec!["pm", "uninstall", "--user", "0", "com.swiftkey.swiftkeyconfigurator"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.mdx"],
            // }}}

            // {{{ SAMSUNG
            // Accessibility
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.accessibility.talkback"],
            // AR
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.app.camera.sticker.facearavatar.preload"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.ardrawing"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.aremoji"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.aremojieditor"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.arzone"],
            // Calendar
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.calendar"],
            // Daydreams
            vec!["pm", "uninstall", "--user", "0", "com.android.dreams.basic"],
            // Free
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.app.spage"],
            // Galaxy Shop
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.galaxy"],
            // Gaming Hub
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.game.gamehome"],
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.game.gametools"],
            // Health
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.app.shealth"],
            // Internet
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.app.sbrowser"],
            // Kids
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.kidsinstaller"],
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.app.kidshome"],
            // Notes
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.app.notes"],
            // Parental control
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.app.parentalcare"],
            // Remote support (Smart tutor)
            vec!["pm", "uninstall", "--user", "0", "com.rsupport.rs.activity.rsupport.aas2"],
            // SmartThings
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.oneconnect"],
            // Stickers
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.stickercenter"],
            // Switch
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.smartswitchassistant"],
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.easyMover"],
            vec!["pm", "uninstall", "--user", "0", "com.sec.android.easyMover.Agent"],
            // Tips
            vec!["pm", "uninstall", "--user", "0", "com.samsung.android.app.tips"],
            // }}}
        ];

        adb_shell(vectors);
    }

    pub fn disable() {
        let vectors: Vec<Vec<&str>> = vec![
            // {{{ GOOGLE
            // Android setup wizard
            vec!["pm", "disable-user", "--user", "0", "com.google.android.setupwizard"],
            // Assistant
            vec!["pm", "disable-user", "--user", "0", "com.android.hotwordenrollment.okgoogle"],
            vec!["pm", "disable-user", "--user", "0", "com.android.hotwordenrollment.xgoogle"],
            // Calendar Sync
            vec!["pm", "disable-user", "--user", "0", "com.google.android.syncadapters.calendar"],
            // One Time Init
            vec!["pm", "disable-user", "--user", "0", "com.google.android.onetimeinitializer"],
            // }}}

            // {{{ SAMSUNG
            // AASAservice
            vec!["pm", "disable-user", "--user", "0", "com.samsung.aasaservice"],
            // Bixby
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.app.settings.bixby"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.bixby.agent"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.bixby.service"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.bixby.wakeup"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.bixbyvision.framework"],
            // Calendar
            vec!["pm", "disable-user", "--user", "0", "com.android.providers.calendar"],
            // Gallery story service
            vec!["pm", "disable-user", "--user", "0", "com.samsung.storyservice"],
            // Link to Windows
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.mdx.kit"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.mdx.quickboard"],
            // Pass
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.samsungpass"],
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.samsungpassautofill"],
            // Visit In
            vec!["pm", "disable-user", "--user", "0", "com.samsung.android.ipsgeofence"],
            // }}}
        ];

        adb_shell(vectors);
    }

    pub fn adb_shell(vectors: Vec<Vec<&str>>) {
        let mut connection = AdbTcpConnection::new(Ipv4Addr::from([127, 0, 0, 1]), 5037).unwrap();

        for vector in vectors.iter() {
            connection.shell_command(&None, vector).unwrap();
            println!("[+] {:?}", vector);
        }
    }
}
