---
title: "Named"
kind: enum
path: "iced_core::keyboard::key::Named"
crate: "iced_core"
module: "iced_core::keyboard::key"
file: "iced_core/keyboard/key/enum.Named.md"
docs_url: "https://docs.iced.rs/iced_core/keyboard/key/enum.Named.html"
source_file: "src/iced_core/keyboard/key.rs"
source_lines: "133-835"
---

# `iced_core::keyboard::key::Named`

```rust
pub enum Named {
Show 306 variants    Alt,
    AltGraph,
    CapsLock,
    Control,
    Fn,
    FnLock,
    NumLock,
    ScrollLock,
    Shift,
    Symbol,
    SymbolLock,
    Meta,
    Hyper,
    Super,
    Enter,
    Tab,
    Space,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
    Backspace,
    Clear,
    Copy,
    CrSel,
    Cut,
    Delete,
    EraseEof,
    ExSel,
    Insert,
    Paste,
    Redo,
    Undo,
    Accept,
    Again,
    Attn,
    Cancel,
    ContextMenu,
    Escape,
    Execute,
    Find,
    Help,
    Pause,
    Play,
    Props,
    Select,
    ZoomIn,
    ZoomOut,
    BrightnessDown,
    BrightnessUp,
    Eject,
    LogOff,
    Power,
    PowerOff,
    PrintScreen,
    Hibernate,
    Standby,
    WakeUp,
    AllCandidates,
    Alphanumeric,
    CodeInput,
    Compose,
    Convert,
    FinalMode,
    GroupFirst,
    GroupLast,
    GroupNext,
    GroupPrevious,
    ModeChange,
    NextCandidate,
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,
    HangulMode,
    HanjaMode,
    JunjaMode,
    Eisu,
    Hankaku,
    Hiragana,
    HiraganaKatakana,
    KanaMode,
    KanjiMode,
    Katakana,
    Romaji,
    Zenkaku,
    ZenkakuHankaku,
    Soft1,
    Soft2,
    Soft3,
    Soft4,
    ChannelDown,
    ChannelUp,
    Close,
    MailForward,
    MailReply,
    MailSend,
    MediaClose,
    MediaFastForward,
    MediaPause,
    MediaPlay,
    MediaPlayPause,
    MediaRecord,
    MediaRewind,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    New,
    Open,
    Print,
    Save,
    SpellCheck,
    Key11,
    Key12,
    AudioBalanceLeft,
    AudioBalanceRight,
    AudioBassBoostDown,
    AudioBassBoostToggle,
    AudioBassBoostUp,
    AudioFaderFront,
    AudioFaderRear,
    AudioSurroundModeNext,
    AudioTrebleDown,
    AudioTrebleUp,
    AudioVolumeDown,
    AudioVolumeUp,
    AudioVolumeMute,
    MicrophoneToggle,
    MicrophoneVolumeDown,
    MicrophoneVolumeUp,
    MicrophoneVolumeMute,
    SpeechCorrectionList,
    SpeechInputToggle,
    LaunchApplication1,
    LaunchApplication2,
    LaunchCalendar,
    LaunchContacts,
    LaunchMail,
    LaunchMediaPlayer,
    LaunchMusicPlayer,
    LaunchPhone,
    LaunchScreenSaver,
    LaunchSpreadsheet,
    LaunchWebBrowser,
    LaunchWebCam,
    LaunchWordProcessor,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    AppSwitch,
    Call,
    Camera,
    CameraFocus,
    EndCall,
    GoBack,
    GoHome,
    HeadsetHook,
    LastNumberRedial,
    Notification,
    MannerMode,
    VoiceDial,
    TV,
    TV3DMode,
    TVAntennaCable,
    TVAudioDescription,
    TVAudioDescriptionMixDown,
    TVAudioDescriptionMixUp,
    TVContentsMenu,
    TVDataService,
    TVInput,
    TVInputComponent1,
    TVInputComponent2,
    TVInputComposite1,
    TVInputComposite2,
    TVInputHDMI1,
    TVInputHDMI2,
    TVInputHDMI3,
    TVInputHDMI4,
    TVInputVGA1,
    TVMediaContext,
    TVNetwork,
    TVNumberEntry,
    TVPower,
    TVRadioService,
    TVSatellite,
    TVSatelliteBS,
    TVSatelliteCS,
    TVSatelliteToggle,
    TVTerrestrialAnalog,
    TVTerrestrialDigital,
    TVTimer,
    AVRInput,
    AVRPower,
    ColorF0Red,
    ColorF1Green,
    ColorF2Yellow,
    ColorF3Blue,
    ColorF4Grey,
    ColorF5Brown,
    ClosedCaptionToggle,
    Dimmer,
    DisplaySwap,
    DVR,
    Exit,
    FavoriteClear0,
    FavoriteClear1,
    FavoriteClear2,
    FavoriteClear3,
    FavoriteRecall0,
    FavoriteRecall1,
    FavoriteRecall2,
    FavoriteRecall3,
    FavoriteStore0,
    FavoriteStore1,
    FavoriteStore2,
    FavoriteStore3,
    Guide,
    GuideNextDay,
    GuidePreviousDay,
    Info,
    InstantReplay,
    Link,
    ListProgram,
    LiveContent,
    Lock,
    MediaApps,
    MediaAudioTrack,
    MediaLast,
    MediaSkipBackward,
    MediaSkipForward,
    MediaStepBackward,
    MediaStepForward,
    MediaTopMenu,
    NavigateIn,
    NavigateNext,
    NavigateOut,
    NavigatePrevious,
    NextFavoriteChannel,
    NextUserProfile,
    OnDemand,
    Pairing,
    PinPDown,
    PinPMove,
    PinPToggle,
    PinPUp,
    PlaySpeedDown,
    PlaySpeedReset,
    PlaySpeedUp,
    RandomToggle,
    RcLowBattery,
    RecordSpeedNext,
    RfBypass,
    ScanChannelsToggle,
    ScreenModeNext,
    Settings,
    SplitScreenToggle,
    STBInput,
    STBPower,
    Subtitle,
    Teletext,
    VideoModeNext,
    Wink,
    ZoomToggle,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}
```

A named key.

This is mostly the `NamedKey` type found in [`winit`](https://docs.rs/winit/0.30/winit/keyboard/enum.Key.html).

**Source:** [key.rs](../../../src/iced_core/keyboard/key.rs.md) (lines 133-835)

## Variants

### `Alt`

The `Alt` (Alternative) key.

This key enables the alternate modifier function for interpreting concurrent or subsequent keyboard input. This key value is also used for the Apple `Option` key.

### `AltGraph`

The Alternate Graphics (`AltGr` or `AltGraph`) key.

This key is used enable the ISO Level 3 shift modifier (the standard `Shift` key is the level 2 modifier).

### `CapsLock`

The `Caps Lock` (Capital) key.

Toggle capital character lock function for interpreting subsequent keyboard input event.

### `Control`

The `Control` or `Ctrl` key.

Used to enable control modifier function for interpreting concurrent or subsequent keyboard input.

### `Fn`

The Function switch `Fn` key. Activating this key simultaneously with another key changes that key’s value to an alternate character or function. This key is often handled directly in the keyboard hardware and does not usually generate key events.

### `FnLock`

The Function-Lock (`FnLock` or `F-Lock`) key. Activating this key switches the mode of the keyboard to changes some keys’ values to an alternate character or function. This key is often handled directly in the keyboard hardware and does not usually generate key events.

### `NumLock`

The `NumLock` or Number Lock key. Used to toggle numpad mode function for interpreting subsequent keyboard input.

### `ScrollLock`

Toggle between scrolling and cursor movement modes.

### `Shift`

Used to enable shift modifier function for interpreting concurrent or subsequent keyboard input.

### `Symbol`

The Symbol modifier key (used on some virtual keyboards).

### `SymbolLock`

### `Meta`

### `Hyper`

### `Super`

Used to enable “super” modifier function for interpreting concurrent or subsequent keyboard input. This key value is used for the “Windows Logo” key and the Apple `Command` or `⌘` key.

Note: In some contexts (e.g. the Web) this is referred to as the “Meta” key.

### `Enter`

The `Enter` or `↵` key. Used to activate current selection or accept current input. This key value is also used for the `Return` (Macintosh numpad) key. This key value is also used for the Android `KEYCODE_DPAD_CENTER`.

### `Tab`

The Horizontal Tabulation `Tab` key.

### `Space`

Used in text to insert a space between words. Usually located below the character keys.

### `ArrowDown`

Navigate or traverse downward. (`KEYCODE_DPAD_DOWN`)

### `ArrowLeft`

Navigate or traverse leftward. (`KEYCODE_DPAD_LEFT`)

### `ArrowRight`

Navigate or traverse rightward. (`KEYCODE_DPAD_RIGHT`)

### `ArrowUp`

Navigate or traverse upward. (`KEYCODE_DPAD_UP`)

### `End`

The End key, used with keyboard entry to go to the end of content (`KEYCODE_MOVE_END`).

### `Home`

The Home key, used with keyboard entry, to go to start of content (`KEYCODE_MOVE_HOME`). For the mobile phone `Home` key (which goes to the phone’s main screen), use [`GoHome`](enum.Named.md).

### `PageDown`

Scroll down or display next page of content.

### `PageUp`

Scroll up or display previous page of content.

### `Backspace`

Used to remove the character to the left of the cursor. This key value is also used for the key labeled `Delete` on MacOS keyboards.

### `Clear`

Remove the currently selected input.

### `Copy`

Copy the current selection. (`APPCOMMAND_COPY`)

### `CrSel`

The Cursor Select key.

### `Cut`

Cut the current selection. (`APPCOMMAND_CUT`)

### `Delete`

Used to delete the character to the right of the cursor. This key value is also used for the key labeled `Delete` on MacOS keyboards when `Fn` is active.

### `EraseEof`

The Erase to End of Field key. This key deletes all characters from the current cursor position to the end of the current field.

### `ExSel`

The Extend Selection (Exsel) key.

### `Insert`

Toggle between text modes for insertion or overtyping. (`KEYCODE_INSERT`)

### `Paste`

The Paste key. (`APPCOMMAND_PASTE`)

### `Redo`

Redo the last action. (`APPCOMMAND_REDO`)

### `Undo`

Undo the last action. (`APPCOMMAND_UNDO`)

### `Accept`

The Accept (Commit, OK) key. Accept current option or input method sequence conversion.

### `Again`

Redo or repeat an action.

### `Attn`

The Attention (Attn) key.

### `Cancel`

### `ContextMenu`

Show the application’s context menu. This key is commonly found between the right `Super` key and the right `Control` key.

### `Escape`

The `Esc` key. This key was originally used to initiate an escape sequence, but is now more generally used to exit or “escape” the current context, such as closing a dialog or exiting full screen mode.

### `Execute`

### `Find`

Open the Find dialog. (`APPCOMMAND_FIND`)

### `Help`

Open a help dialog or toggle display of help information. (`APPCOMMAND_HELP`, `KEYCODE_HELP`)

### `Pause`

Pause the current state or application (as appropriate).

Note: Do not use this value for the `Pause` button on media controllers. Use `"MediaPause"` instead.

### `Play`

Play or resume the current state or application (as appropriate).

Note: Do not use this value for the `Play` button on media controllers. Use `"MediaPlay"` instead.

### `Props`

The properties (Props) key.

### `Select`

### `ZoomIn`

The ZoomIn key. (`KEYCODE_ZOOM_IN`)

### `ZoomOut`

The ZoomOut key. (`KEYCODE_ZOOM_OUT`)

### `BrightnessDown`

The Brightness Down key. Typically controls the display brightness. (`KEYCODE_BRIGHTNESS_DOWN`)

### `BrightnessUp`

The Brightness Up key. Typically controls the display brightness. (`KEYCODE_BRIGHTNESS_UP`)

### `Eject`

Toggle removable media to eject (open) and insert (close) state. (`KEYCODE_MEDIA_EJECT`)

### `LogOff`

### `Power`

Toggle power state. (`KEYCODE_POWER`) Note: Note: Some devices might not expose this key to the operating environment.

### `PowerOff`

The `PowerOff` key. Sometime called `PowerDown`.

### `PrintScreen`

Initiate print-screen function.

### `Hibernate`

The Hibernate key. This key saves the current state of the computer to disk so that it can be restored. The computer will then shutdown.

### `Standby`

The Standby key. This key turns off the display and places the computer into a low-power mode without completely shutting down. It is sometimes labelled `Suspend` or `Sleep` key. (`KEYCODE_SLEEP`)

### `WakeUp`

The WakeUp key. (`KEYCODE_WAKEUP`)

### `AllCandidates`

Initiate the multi-candidate mode.

### `Alphanumeric`

### `CodeInput`

Initiate the Code Input mode to allow characters to be entered by their code points.

### `Compose`

The Compose key, also known as “Multi_key” on the X Window System. This key acts in a manner similar to a dead key, triggering a mode where subsequent key presses are combined to produce a different character.

### `Convert`

Convert the current input method sequence.

### `FinalMode`

The Final Mode `Final` key used on some Asian keyboards, to enable the final mode for IMEs.

### `GroupFirst`

Switch to the first character group. (ISO/IEC 9995)

### `GroupLast`

Switch to the last character group. (ISO/IEC 9995)

### `GroupNext`

Switch to the next character group. (ISO/IEC 9995)

### `GroupPrevious`

Switch to the previous character group. (ISO/IEC 9995)

### `ModeChange`

Toggle between or cycle through input modes of IMEs.

### `NextCandidate`

### `NonConvert`

Accept current input method sequence without conversion in IMEs.

### `PreviousCandidate`

### `Process`

### `SingleCandidate`

### `HangulMode`

Toggle between Hangul and English modes.

### `HanjaMode`

### `JunjaMode`

### `Eisu`

The Eisu key. This key may close the IME, but its purpose is defined by the current IME. (`KEYCODE_EISU`)

### `Hankaku`

The (Half-Width) Characters key.

### `Hiragana`

The Hiragana (Japanese Kana characters) key.

### `HiraganaKatakana`

The Hiragana/Katakana toggle key. (`KEYCODE_KATAKANA_HIRAGANA`)

### `KanaMode`

The Kana Mode (Kana Lock) key. This key is used to enter hiragana mode (typically from romaji mode).

### `KanjiMode`

The Kanji (Japanese name for ideographic characters of Chinese origin) Mode key. This key is typically used to switch to a hiragana keyboard for the purpose of converting input into kanji. (`KEYCODE_KANA`)

### `Katakana`

The Katakana (Japanese Kana characters) key.

### `Romaji`

The Roman characters function key.

### `Zenkaku`

The Zenkaku (Full-Width) Characters key.

### `ZenkakuHankaku`

The Zenkaku/Hankaku (full-width/half-width) toggle key. (`KEYCODE_ZENKAKU_HANKAKU`)

### `Soft1`

General purpose virtual function key, as index 1.

### `Soft2`

General purpose virtual function key, as index 2.

### `Soft3`

General purpose virtual function key, as index 3.

### `Soft4`

General purpose virtual function key, as index 4.

### `ChannelDown`

Select next (numerically or logically) lower channel. (`APPCOMMAND_MEDIA_CHANNEL_DOWN`, `KEYCODE_CHANNEL_DOWN`)

### `ChannelUp`

Select next (numerically or logically) higher channel. (`APPCOMMAND_MEDIA_CHANNEL_UP`, `KEYCODE_CHANNEL_UP`)

### `Close`

Close the current document or message (Note: This doesn’t close the application). (`APPCOMMAND_CLOSE`)

### `MailForward`

Open an editor to forward the current message. (`APPCOMMAND_FORWARD_MAIL`)

### `MailReply`

Open an editor to reply to the current message. (`APPCOMMAND_REPLY_TO_MAIL`)

### `MailSend`

Send the current message. (`APPCOMMAND_SEND_MAIL`)

### `MediaClose`

Close the current media, for example to close a CD or DVD tray. (`KEYCODE_MEDIA_CLOSE`)

### `MediaFastForward`

Initiate or continue forward playback at faster than normal speed, or increase speed if already fast forwarding. (`APPCOMMAND_MEDIA_FAST_FORWARD`, `KEYCODE_MEDIA_FAST_FORWARD`)

### `MediaPause`

Pause the currently playing media. (`APPCOMMAND_MEDIA_PAUSE`, `KEYCODE_MEDIA_PAUSE`)

Note: Media controller devices should use this value rather than `"Pause"` for their pause keys.

### `MediaPlay`

Initiate or continue media playback at normal speed, if not currently playing at normal speed. (`APPCOMMAND_MEDIA_PLAY`, `KEYCODE_MEDIA_PLAY`)

### `MediaPlayPause`

Toggle media between play and pause states. (`APPCOMMAND_MEDIA_PLAY_PAUSE`, `KEYCODE_MEDIA_PLAY_PAUSE`)

### `MediaRecord`

Initiate or resume recording of currently selected media. (`APPCOMMAND_MEDIA_RECORD`, `KEYCODE_MEDIA_RECORD`)

### `MediaRewind`

Initiate or continue reverse playback at faster than normal speed, or increase speed if already rewinding. (`APPCOMMAND_MEDIA_REWIND`, `KEYCODE_MEDIA_REWIND`)

### `MediaStop`

Stop media playing, pausing, forwarding, rewinding, or recording, if not already stopped. (`APPCOMMAND_MEDIA_STOP`, `KEYCODE_MEDIA_STOP`)

### `MediaTrackNext`

Seek to next media or program track. (`APPCOMMAND_MEDIA_NEXTTRACK`, `KEYCODE_MEDIA_NEXT`)

### `MediaTrackPrevious`

Seek to previous media or program track. (`APPCOMMAND_MEDIA_PREVIOUSTRACK`, `KEYCODE_MEDIA_PREVIOUS`)

### `New`

Open a new document or message. (`APPCOMMAND_NEW`)

### `Open`

Open an existing document or message. (`APPCOMMAND_OPEN`)

### `Print`

Print the current document or message. (`APPCOMMAND_PRINT`)

### `Save`

Save the current document or message. (`APPCOMMAND_SAVE`)

### `SpellCheck`

Spellcheck the current document or selection. (`APPCOMMAND_SPELL_CHECK`)

### `Key11`

The `11` key found on media numpads that have buttons from `1` … `12`.

### `Key12`

The `12` key found on media numpads that have buttons from `1` … `12`.

### `AudioBalanceLeft`

Adjust audio balance leftward. (`VK_AUDIO_BALANCE_LEFT`)

### `AudioBalanceRight`

Adjust audio balance rightward. (`VK_AUDIO_BALANCE_RIGHT`)

### `AudioBassBoostDown`

Decrease audio bass boost or cycle down through bass boost states. (`APPCOMMAND_BASS_DOWN`, `VK_BASS_BOOST_DOWN`)

### `AudioBassBoostToggle`

Toggle bass boost on/off. (`APPCOMMAND_BASS_BOOST`)

### `AudioBassBoostUp`

Increase audio bass boost or cycle up through bass boost states. (`APPCOMMAND_BASS_UP`, `VK_BASS_BOOST_UP`)

### `AudioFaderFront`

Adjust audio fader towards front. (`VK_FADER_FRONT`)

### `AudioFaderRear`

Adjust audio fader towards rear. (`VK_FADER_REAR`)

### `AudioSurroundModeNext`

Advance surround audio mode to next available mode. (`VK_SURROUND_MODE_NEXT`)

### `AudioTrebleDown`

Decrease treble. (`APPCOMMAND_TREBLE_DOWN`)

### `AudioTrebleUp`

Increase treble. (`APPCOMMAND_TREBLE_UP`)

### `AudioVolumeDown`

Decrease audio volume. (`APPCOMMAND_VOLUME_DOWN`, `KEYCODE_VOLUME_DOWN`)

### `AudioVolumeUp`

Increase audio volume. (`APPCOMMAND_VOLUME_UP`, `KEYCODE_VOLUME_UP`)

### `AudioVolumeMute`

Toggle between muted state and prior volume level. (`APPCOMMAND_VOLUME_MUTE`, `KEYCODE_VOLUME_MUTE`)

### `MicrophoneToggle`

Toggle the microphone on/off. (`APPCOMMAND_MIC_ON_OFF_TOGGLE`)

### `MicrophoneVolumeDown`

Decrease microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_DOWN`)

### `MicrophoneVolumeUp`

Increase microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_UP`)

### `MicrophoneVolumeMute`

Mute the microphone. (`APPCOMMAND_MICROPHONE_VOLUME_MUTE`, `KEYCODE_MUTE`)

### `SpeechCorrectionList`

Show correction list when a word is incorrectly identified. (`APPCOMMAND_CORRECTION_LIST`)

### `SpeechInputToggle`

Toggle between dictation mode and command/control mode. (`APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE`)

### `LaunchApplication1`

The first generic “LaunchApplication” key. This is commonly associated with launching “My Computer”, and may have a computer symbol on the key. (`APPCOMMAND_LAUNCH_APP1`)

### `LaunchApplication2`

The second generic “LaunchApplication” key. This is commonly associated with launching “Calculator”, and may have a calculator symbol on the key. (`APPCOMMAND_LAUNCH_APP2`, `KEYCODE_CALCULATOR`)

### `LaunchCalendar`

The “Calendar” key. (`KEYCODE_CALENDAR`)

### `LaunchContacts`

The “Contacts” key. (`KEYCODE_CONTACTS`)

### `LaunchMail`

The “Mail” key. (`APPCOMMAND_LAUNCH_MAIL`)

### `LaunchMediaPlayer`

The “Media Player” key. (`APPCOMMAND_LAUNCH_MEDIA_SELECT`)

### `LaunchMusicPlayer`

### `LaunchPhone`

### `LaunchScreenSaver`

### `LaunchSpreadsheet`

### `LaunchWebBrowser`

### `LaunchWebCam`

### `LaunchWordProcessor`

### `BrowserBack`

Navigate to previous content or page in current history. (`APPCOMMAND_BROWSER_BACKWARD`)

### `BrowserFavorites`

Open the list of browser favorites. (`APPCOMMAND_BROWSER_FAVORITES`)

### `BrowserForward`

Navigate to next content or page in current history. (`APPCOMMAND_BROWSER_FORWARD`)

### `BrowserHome`

Go to the user’s preferred home page. (`APPCOMMAND_BROWSER_HOME`)

### `BrowserRefresh`

Refresh the current page or content. (`APPCOMMAND_BROWSER_REFRESH`)

### `BrowserSearch`

Call up the user’s preferred search page. (`APPCOMMAND_BROWSER_SEARCH`)

### `BrowserStop`

Stop loading the current page or content. (`APPCOMMAND_BROWSER_STOP`)

### `AppSwitch`

The Application switch key, which provides a list of recent apps to switch between. (`KEYCODE_APP_SWITCH`)

### `Call`

The Call key. (`KEYCODE_CALL`)

### `Camera`

The Camera key. (`KEYCODE_CAMERA`)

### `CameraFocus`

The Camera focus key. (`KEYCODE_FOCUS`)

### `EndCall`

The End Call key. (`KEYCODE_ENDCALL`)

### `GoBack`

The Back key. (`KEYCODE_BACK`)

### `GoHome`

The Home key, which goes to the phone’s main screen. (`KEYCODE_HOME`)

### `HeadsetHook`

The Headset Hook key. (`KEYCODE_HEADSETHOOK`)

### `LastNumberRedial`

### `Notification`

The Notification key. (`KEYCODE_NOTIFICATION`)

### `MannerMode`

Toggle between manner mode state: silent, vibrate, ring, … (`KEYCODE_MANNER_MODE`)

### `VoiceDial`

### `TV`

Switch to viewing TV. (`KEYCODE_TV`)

### `TV3DMode`

TV 3D Mode. (`KEYCODE_3D_MODE`)

### `TVAntennaCable`

Toggle between antenna and cable input. (`KEYCODE_TV_ANTENNA_CABLE`)

### `TVAudioDescription`

Audio description. (`KEYCODE_TV_AUDIO_DESCRIPTION`)

### `TVAudioDescriptionMixDown`

Audio description mixing volume down. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN`)

### `TVAudioDescriptionMixUp`

Audio description mixing volume up. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP`)

### `TVContentsMenu`

Contents menu. (`KEYCODE_TV_CONTENTS_MENU`)

### `TVDataService`

Contents menu. (`KEYCODE_TV_DATA_SERVICE`)

### `TVInput`

Switch the input mode on an external TV. (`KEYCODE_TV_INPUT`)

### `TVInputComponent1`

Switch to component input #1. (`KEYCODE_TV_INPUT_COMPONENT_1`)

### `TVInputComponent2`

Switch to component input #2. (`KEYCODE_TV_INPUT_COMPONENT_2`)

### `TVInputComposite1`

Switch to composite input #1. (`KEYCODE_TV_INPUT_COMPOSITE_1`)

### `TVInputComposite2`

Switch to composite input #2. (`KEYCODE_TV_INPUT_COMPOSITE_2`)

### `TVInputHDMI1`

Switch to HDMI input #1. (`KEYCODE_TV_INPUT_HDMI_1`)

### `TVInputHDMI2`

Switch to HDMI input #2. (`KEYCODE_TV_INPUT_HDMI_2`)

### `TVInputHDMI3`

Switch to HDMI input #3. (`KEYCODE_TV_INPUT_HDMI_3`)

### `TVInputHDMI4`

Switch to HDMI input #4. (`KEYCODE_TV_INPUT_HDMI_4`)

### `TVInputVGA1`

Switch to VGA input #1. (`KEYCODE_TV_INPUT_VGA_1`)

### `TVMediaContext`

Media context menu. (`KEYCODE_TV_MEDIA_CONTEXT_MENU`)

### `TVNetwork`

Toggle network. (`KEYCODE_TV_NETWORK`)

### `TVNumberEntry`

Number entry. (`KEYCODE_TV_NUMBER_ENTRY`)

### `TVPower`

Toggle the power on an external TV. (`KEYCODE_TV_POWER`)

### `TVRadioService`

Radio. (`KEYCODE_TV_RADIO_SERVICE`)

### `TVSatellite`

Satellite. (`KEYCODE_TV_SATELLITE`)

### `TVSatelliteBS`

Broadcast Satellite. (`KEYCODE_TV_SATELLITE_BS`)

### `TVSatelliteCS`

Communication Satellite. (`KEYCODE_TV_SATELLITE_CS`)

### `TVSatelliteToggle`

Toggle between available satellites. (`KEYCODE_TV_SATELLITE_SERVICE`)

### `TVTerrestrialAnalog`

Analog Terrestrial. (`KEYCODE_TV_TERRESTRIAL_ANALOG`)

### `TVTerrestrialDigital`

Digital Terrestrial. (`KEYCODE_TV_TERRESTRIAL_DIGITAL`)

### `TVTimer`

Timer programming. (`KEYCODE_TV_TIMER_PROGRAMMING`)

### `AVRInput`

Switch the input mode on an external AVR (audio/video receiver). (`KEYCODE_AVR_INPUT`)

### `AVRPower`

Toggle the power on an external AVR (audio/video receiver). (`KEYCODE_AVR_POWER`)

### `ColorF0Red`

General purpose color-coded media function key, as index 0 (red). (`VK_COLORED_KEY_0`, `KEYCODE_PROG_RED`)

### `ColorF1Green`

General purpose color-coded media function key, as index 1 (green). (`VK_COLORED_KEY_1`, `KEYCODE_PROG_GREEN`)

### `ColorF2Yellow`

General purpose color-coded media function key, as index 2 (yellow). (`VK_COLORED_KEY_2`, `KEYCODE_PROG_YELLOW`)

### `ColorF3Blue`

General purpose color-coded media function key, as index 3 (blue). (`VK_COLORED_KEY_3`, `KEYCODE_PROG_BLUE`)

### `ColorF4Grey`

General purpose color-coded media function key, as index 4 (grey). (`VK_COLORED_KEY_4`)

### `ColorF5Brown`

General purpose color-coded media function key, as index 5 (brown). (`VK_COLORED_KEY_5`)

### `ClosedCaptionToggle`

Toggle the display of Closed Captions. (`VK_CC`, `KEYCODE_CAPTIONS`)

### `Dimmer`

Adjust brightness of device, by toggling between or cycling through states. (`VK_DIMMER`)

### `DisplaySwap`

Swap video sources. (`VK_DISPLAY_SWAP`)

### `DVR`

Select Digital Video Rrecorder. (`KEYCODE_DVR`)

### `Exit`

Exit the current application. (`VK_EXIT`)

### `FavoriteClear0`

Clear program or content stored as favorite 0. (`VK_CLEAR_FAVORITE_0`)

### `FavoriteClear1`

Clear program or content stored as favorite 1. (`VK_CLEAR_FAVORITE_1`)

### `FavoriteClear2`

Clear program or content stored as favorite 2. (`VK_CLEAR_FAVORITE_2`)

### `FavoriteClear3`

Clear program or content stored as favorite 3. (`VK_CLEAR_FAVORITE_3`)

### `FavoriteRecall0`

Select (recall) program or content stored as favorite 0. (`VK_RECALL_FAVORITE_0`)

### `FavoriteRecall1`

Select (recall) program or content stored as favorite 1. (`VK_RECALL_FAVORITE_1`)

### `FavoriteRecall2`

Select (recall) program or content stored as favorite 2. (`VK_RECALL_FAVORITE_2`)

### `FavoriteRecall3`

Select (recall) program or content stored as favorite 3. (`VK_RECALL_FAVORITE_3`)

### `FavoriteStore0`

Store current program or content as favorite 0. (`VK_STORE_FAVORITE_0`)

### `FavoriteStore1`

Store current program or content as favorite 1. (`VK_STORE_FAVORITE_1`)

### `FavoriteStore2`

Store current program or content as favorite 2. (`VK_STORE_FAVORITE_2`)

### `FavoriteStore3`

Store current program or content as favorite 3. (`VK_STORE_FAVORITE_3`)

### `Guide`

Toggle display of program or content guide. (`VK_GUIDE`, `KEYCODE_GUIDE`)

### `GuideNextDay`

If guide is active and displayed, then display next day’s content. (`VK_NEXT_DAY`)

### `GuidePreviousDay`

If guide is active and displayed, then display previous day’s content. (`VK_PREV_DAY`)

### `Info`

Toggle display of information about currently selected context or media. (`VK_INFO`, `KEYCODE_INFO`)

### `InstantReplay`

Toggle instant replay. (`VK_INSTANT_REPLAY`)

### `Link`

Launch linked content, if available and appropriate. (`VK_LINK`)

### `ListProgram`

List the current program. (`VK_LIST`)

### `LiveContent`

Toggle display listing of currently available live content or programs. (`VK_LIVE`)

### `Lock`

Lock or unlock current content or program. (`VK_LOCK`)

### `MediaApps`

Show a list of media applications: audio/video players and image viewers. (`VK_APPS`)

Note: Do not confuse this key value with the Windows’ `VK_APPS` / `VK_CONTEXT_MENU` key, which is encoded as `"ContextMenu"`.

### `MediaAudioTrack`

Audio track key. (`KEYCODE_MEDIA_AUDIO_TRACK`)

### `MediaLast`

Select previously selected channel or media. (`VK_LAST`, `KEYCODE_LAST_CHANNEL`)

### `MediaSkipBackward`

Skip backward to next content or program. (`KEYCODE_MEDIA_SKIP_BACKWARD`)

### `MediaSkipForward`

Skip forward to next content or program. (`VK_SKIP`, `KEYCODE_MEDIA_SKIP_FORWARD`)

### `MediaStepBackward`

Step backward to next content or program. (`KEYCODE_MEDIA_STEP_BACKWARD`)

### `MediaStepForward`

Step forward to next content or program. (`KEYCODE_MEDIA_STEP_FORWARD`)

### `MediaTopMenu`

Media top menu. (`KEYCODE_MEDIA_TOP_MENU`)

### `NavigateIn`

Navigate in. (`KEYCODE_NAVIGATE_IN`)

### `NavigateNext`

Navigate to next key. (`KEYCODE_NAVIGATE_NEXT`)

### `NavigateOut`

Navigate out. (`KEYCODE_NAVIGATE_OUT`)

### `NavigatePrevious`

Navigate to previous key. (`KEYCODE_NAVIGATE_PREVIOUS`)

### `NextFavoriteChannel`

Cycle to next favorite channel (in favorites list). (`VK_NEXT_FAVORITE_CHANNEL`)

### `NextUserProfile`

Cycle to next user profile (if there are multiple user profiles). (`VK_USER`)

### `OnDemand`

Access on-demand content or programs. (`VK_ON_DEMAND`)

### `Pairing`

Pairing key to pair devices. (`KEYCODE_PAIRING`)

### `PinPDown`

Move picture-in-picture window down. (`VK_PINP_DOWN`)

### `PinPMove`

Move picture-in-picture window. (`VK_PINP_MOVE`)

### `PinPToggle`

Toggle display of picture-in-picture window. (`VK_PINP_TOGGLE`)

### `PinPUp`

Move picture-in-picture window up. (`VK_PINP_UP`)

### `PlaySpeedDown`

Decrease media playback speed. (`VK_PLAY_SPEED_DOWN`)

### `PlaySpeedReset`

Reset playback to normal speed. (`VK_PLAY_SPEED_RESET`)

### `PlaySpeedUp`

Increase media playback speed. (`VK_PLAY_SPEED_UP`)

### `RandomToggle`

Toggle random media or content shuffle mode. (`VK_RANDOM_TOGGLE`)

### `RcLowBattery`

Not a physical key, but this key code is sent when the remote control battery is low. (`VK_RC_LOW_BATTERY`)

### `RecordSpeedNext`

Toggle or cycle between media recording speeds. (`VK_RECORD_SPEED_NEXT`)

### `RfBypass`

Toggle RF (radio frequency) input bypass mode (pass RF input directly to the RF output). (`VK_RF_BYPASS`)

### `ScanChannelsToggle`

Toggle scan channels mode. (`VK_SCAN_CHANNELS_TOGGLE`)

### `ScreenModeNext`

Advance display screen mode to next available mode. (`VK_SCREEN_MODE_NEXT`)

### `Settings`

Toggle display of device settings screen. (`VK_SETTINGS`, `KEYCODE_SETTINGS`)

### `SplitScreenToggle`

Toggle split screen mode. (`VK_SPLIT_SCREEN_TOGGLE`)

### `STBInput`

Switch the input mode on an external STB (set top box). (`KEYCODE_STB_INPUT`)

### `STBPower`

Toggle the power on an external STB (set top box). (`KEYCODE_STB_POWER`)

### `Subtitle`

Toggle display of subtitles, if available. (`VK_SUBTITLE`)

### `Teletext`

Toggle display of teletext, if available (`VK_TELETEXT`, `KEYCODE_TV_TELETEXT`).

### `VideoModeNext`

Advance video mode to next available mode. (`VK_VIDEO_MODE_NEXT`)

### `Wink`

Cause device to identify itself in some manner, e.g., audibly or visibly. (`VK_WINK`)

### `ZoomToggle`

Toggle between full-screen and scaled content, or alter magnification level. (`VK_ZOOM`, `KEYCODE_TV_ZOOM_MODE`)

### `F1`

General-purpose function key. Usually found at the top of the keyboard.

### `F2`

General-purpose function key. Usually found at the top of the keyboard.

### `F3`

General-purpose function key. Usually found at the top of the keyboard.

### `F4`

General-purpose function key. Usually found at the top of the keyboard.

### `F5`

General-purpose function key. Usually found at the top of the keyboard.

### `F6`

General-purpose function key. Usually found at the top of the keyboard.

### `F7`

General-purpose function key. Usually found at the top of the keyboard.

### `F8`

General-purpose function key. Usually found at the top of the keyboard.

### `F9`

General-purpose function key. Usually found at the top of the keyboard.

### `F10`

General-purpose function key. Usually found at the top of the keyboard.

### `F11`

General-purpose function key. Usually found at the top of the keyboard.

### `F12`

General-purpose function key. Usually found at the top of the keyboard.

### `F13`

General-purpose function key. Usually found at the top of the keyboard.

### `F14`

General-purpose function key. Usually found at the top of the keyboard.

### `F15`

General-purpose function key. Usually found at the top of the keyboard.

### `F16`

General-purpose function key. Usually found at the top of the keyboard.

### `F17`

General-purpose function key. Usually found at the top of the keyboard.

### `F18`

General-purpose function key. Usually found at the top of the keyboard.

### `F19`

General-purpose function key. Usually found at the top of the keyboard.

### `F20`

General-purpose function key. Usually found at the top of the keyboard.

### `F21`

General-purpose function key. Usually found at the top of the keyboard.

### `F22`

General-purpose function key. Usually found at the top of the keyboard.

### `F23`

General-purpose function key. Usually found at the top of the keyboard.

### `F24`

General-purpose function key. Usually found at the top of the keyboard.

### `F25`

General-purpose function key.

### `F26`

General-purpose function key.

### `F27`

General-purpose function key.

### `F28`

General-purpose function key.

### `F29`

General-purpose function key.

### `F30`

General-purpose function key.

### `F31`

General-purpose function key.

### `F32`

General-purpose function key.

### `F33`

General-purpose function key.

### `F34`

General-purpose function key.

### `F35`

General-purpose function key.

## Trait Implementations

### `impl Clone for Named`

#### `clone`

```rust
fn clone(&self) -> Named
```

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

#### `clone_from`

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

### `impl Debug for Named`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl From<Named> for Key`

#### `from`

```rust
fn from(named: Named) -> Self
```

Converts to this type from the input type.

### `impl Hash for Named`

#### `hash`

```rust
fn hash<__H: Hasher>(&self, state: &mut __H)
```

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

#### `hash_slice`

```rust
fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher, Self: Sized,
```

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

### `impl Ord for Named`

#### `cmp`

```rust
fn cmp(&self, other: &Named) -> Ordering
```

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

#### `max`

```rust
fn max(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

#### `min`

```rust
fn min(self, other: Self) -> Self where Self: Sized,
```

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

#### `clamp`

```rust
fn clamp(self, min: Self, max: Self) -> Self where Self: Sized,
```

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

### `impl PartialEq for Named`

#### `eq`

```rust
fn eq(&self, other: &Named) -> bool
```

Tests for `self` and `other` values to be equal, and is used by `==`.

#### `ne`

```rust
fn ne(&self, other: &Rhs) -> bool
```

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

### `impl PartialOrd for Named`

#### `partial_cmp`

```rust
fn partial_cmp(&self, other: &Named) -> Option<Ordering>
```

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

#### `lt`

```rust
fn lt(&self, other: &Rhs) -> bool
```

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

#### `le`

```rust
fn le(&self, other: &Rhs) -> bool
```

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

#### `gt`

```rust
fn gt(&self, other: &Rhs) -> bool
```

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

#### `ge`

```rust
fn ge(&self, other: &Rhs) -> bool
```

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

### `impl Copy for Named`

### `impl Eq for Named`

### `impl StructuralPartialEq for Named`

## Auto Trait Implementations

- `impl Freeze for Named`
- `impl RefUnwindSafe for Named`
- `impl Send for Named`
- `impl Sync for Named`
- `impl Unpin for Named`
- `impl UnsafeUnpin for Named`
- `impl UnwindSafe for Named`
## Blanket Implementations

### `impl<T> Any for T where T: 'static + ?Sized,`

#### `type_id`

```rust
fn type_id(&self) -> TypeId
```

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

### `impl<T> Borrow<T> for T where T: ?Sized,`

#### `borrow`

```rust
fn borrow(&self) -> &T
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<T> BorrowMut<T> for T where T: ?Sized,`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut T
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<T> CloneToUninit for T where T: Clone,`

#### `clone_to_uninit`

```rust
unsafe fn clone_to_uninit(&self, dest: *mut u8)
```

*🔬This is a nightly-only experimental API. (clone_to_uninit)*

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<T> ToOwned for T where T: Clone,`

#### `Owned`

```rust
type Owned = T
```

The resulting type after obtaining ownership.

#### `to_owned`

```rust
fn to_owned(&self) -> T
```

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

#### `clone_into`

```rust
fn clone_into(&self, target: &mut T)
```

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

### `impl<T, U> TryFrom<U> for T where U: Into<T>,`

#### `Error`

```rust
type Error = Infallible
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
```

Performs the conversion.

### `impl<T, U> TryInto<U> for T where U: TryFrom<T>,`

#### `Error`

```rust
type Error = <U as TryFrom<T>>::Error
```

The type returned in the event of a conversion error.

#### `try_into`

```rust
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
```

Performs the conversion.
