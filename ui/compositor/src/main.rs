use iced::widget::{button, column, container, row, text, Space, text_input};
use iced::{Element, Length, Color, Size, window, Border};

pub fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Sarcophagus OS - Secure Shell")
        .window(window::Settings {
            size: Size { width: 390.0, height: 800.0 },
            resizable: false,
            decorations: true,
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ActiveSpace {
    #[default]
    Personal,
    Work,
    Private,
}

struct State {
    current_space: ActiveSpace,
    search_query: String,
    trackers_blocked: u64,
    mic_kill_switch: bool,
    camera_kill_switch: bool,
    location_kill_switch: bool,
    show_shield_panel: bool,
    show_spaces_panel: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_space: ActiveSpace::Personal,
            search_query: String::new(),
            trackers_blocked: 128,
            mic_kill_switch: false,
            camera_kill_switch: false,
            location_kill_switch: false,
            show_shield_panel: false,
            show_spaces_panel: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SwitchSpace(ActiveSpace),
    SearchChanged(String),
    ToggleShieldPanel,
    ToggleSpacesPanel,
    ToggleMicKill,
    ToggleCameraKill,
    ToggleLocationKill,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::SwitchSpace(space) => {
            state.current_space = space;
            state.show_spaces_panel = false;
        }
        Message::SearchChanged(query) => {
            state.search_query = query;
        }
        Message::ToggleShieldPanel => {
            state.show_shield_panel = !state.show_shield_panel;
            state.show_spaces_panel = false;
        }
        Message::ToggleSpacesPanel => {
            state.show_spaces_panel = !state.show_spaces_panel;
            state.show_shield_panel = false;
        }
        Message::ToggleMicKill => {
            state.mic_kill_switch = !state.mic_kill_switch;
        }
        Message::ToggleCameraKill => {
            state.camera_kill_switch = !state.camera_kill_switch;
        }
        Message::ToggleLocationKill => {
            state.location_kill_switch = !state.location_kill_switch;
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let accent_color = match state.current_space {
        ActiveSpace::Personal => Color::from_rgb(0.0, 0.6, 1.0),
        ActiveSpace::Work     => Color::from_rgb(1.0, 0.7, 0.0),
        ActiveSpace::Private  => Color::from_rgb(0.8, 0.2, 1.0),
    };

    let content = if state.show_shield_panel {
        // --- SHIELD PANEL VIEW ---
        column![
            row![
                text("🛡 SHIELD CONTROL").size(16).color(Color::WHITE),
                Space::new().width(Length::Fill),
                button("✕").padding(6).on_press(Message::ToggleShieldPanel)
            ],
            Space::new().height(Length::Fixed(15.0)),
            
            container(
                column![
                    text("ACTIVE SENSOR AUDIT").size(10).color(Color::from_rgb(0.5, 0.55, 0.65)),
                    Space::new().height(Length::Fixed(6.0)),
                    row![
                        text("Microphone (Signal)").size(13).color(Color::from_rgb(1.0, 0.6, 0.0)),
                        Space::new().width(Length::Fill),
                        text(if state.mic_kill_switch { "BLOCKED" } else { "ACTIVE" }).size(11)
                            .color(if state.mic_kill_switch { Color::from_rgb(1.0, 0.3, 0.3) } else { Color::from_rgb(0.0, 1.0, 0.5) }),
                    ],
                ]
            ).padding(12).width(Length::Fill).style(|_| container::Style {
                background: Some(Color::from_rgb(0.1, 0.12, 0.16).into()),
                border: Border { color: Color::from_rgb(0.2, 0.25, 0.35), width: 1.0, radius: 8.0.into() },
                ..Default::default()
            }),

            Space::new().height(Length::Fixed(15.0)),
            text("HARDWARE KILL SWITCHES").size(10).color(Color::from_rgb(0.5, 0.55, 0.65)),
            Space::new().height(Length::Fixed(8.0)),

            button(
                row![text("Microphone Access").size(13), Space::new().width(Length::Fill), text(if state.mic_kill_switch { "OFF" } else { "ON" })]
            ).padding(12).width(Length::Fill).on_press(Message::ToggleMicKill),

            Space::new().height(Length::Fixed(8.0)),
            button(
                row![text("Camera Access").size(13), Space::new().width(Length::Fill), text(if state.camera_kill_switch { "OFF" } else { "ON" })]
            ).padding(12).width(Length::Fill).on_press(Message::ToggleCameraKill),

            Space::new().height(Length::Fixed(8.0)),
            button(
                row![text("Location Gateway").size(13), Space::new().width(Length::Fill), text(if state.location_kill_switch { "OFF" } else { "ON" })]
            ).padding(12).width(Length::Fill).on_press(Message::ToggleLocationKill),

            Space::new().height(Length::Fill),
            container(text("✓ No background apps accessed sensors in last hour").size(11).color(Color::from_rgb(0.4, 0.8, 0.5)))
                .padding(10).width(Length::Fill)
                .style(|_| container::Style { 
                    background: Some(Color::from_rgb(0.07, 0.14, 0.09).into()), 
                    border: Border { radius: 6.0.into(), ..Default::default() }, 
                    ..Default::default() 
                })
        ]
    } else if state.show_spaces_panel {
        // --- SPACES SWITCHER VIEW ---
        column![
            row![
                text("ISOLATED SPACES").size(16).color(Color::WHITE),
                Space::new().width(Length::Fill),
                button("✕").padding(6).on_press(Message::ToggleSpacesPanel)
            ],
            Space::new().height(Length::Fixed(20.0)),

            space_card("Personal", "24 apps, normal network", state.current_space == ActiveSpace::Personal, Color::from_rgb(0.0, 0.6, 1.0), Message::SwitchSpace(ActiveSpace::Personal)),
            Space::new().height(Length::Fixed(10.0)),
            space_card("Work", "9 apps, company VPN", state.current_space == ActiveSpace::Work, Color::from_rgb(1.0, 0.7, 0.0), Message::SwitchSpace(ActiveSpace::Work)),
            Space::new().height(Length::Fixed(10.0)),
            space_card("Private", "3 apps, Tor-only routing", state.current_space == ActiveSpace::Private, Color::from_rgb(0.8, 0.2, 1.0), Message::SwitchSpace(ActiveSpace::Private)),

            Space::new().height(Length::Fill),
            button(container(text("+ New Space").size(13)).width(Length::Fill).center_x(Length::Fill))
                .padding(12).width(Length::Fill).on_press(Message::ToggleSpacesPanel)
        ]
    } else {
        // --- HOME SCREEN (THUMB-ZONE FIRST) ---
        column![
            // Top Glances (Read-Only)
            row![
                text("09:41").size(28).color(Color::WHITE),
                Space::new().width(Length::Fill),
                button(text("● Signal").size(11)).padding(5).on_press(Message::ToggleShieldPanel)
                    .style(|_, _| button::Style { 
                        background: Some(Color::from_rgb(0.2, 0.12, 0.02).into()), 
                        border: Border { radius: 10.0.into(), ..Default::default() },
                        text_color: Color::from_rgb(1.0, 0.6, 0.0), 
                        ..Default::default() 
                    })
            ],
            text("Sunday, 11 October").size(12).color(Color::from_rgb(0.5, 0.55, 0.65)),

            Space::new().height(Length::Fixed(16.0)),

            // Glance Card: Next Up
            container(
                column![
                    text("NEXT UP").size(9).color(Color::from_rgb(0.4, 0.5, 0.6)),
                    Space::new().height(Length::Fixed(4.0)),
                    text("Design sync, 10:30").size(14).color(Color::WHITE),
                ]
            ).padding(14).width(Length::Fill).style(|_| container::Style {
                background: Some(Color::from_rgb(0.09, 0.11, 0.15).into()),
                border: Border { color: Color::from_rgb(0.18, 0.22, 0.3), width: 1.0, radius: 12.0.into() },
                ..Default::default()
            }),

            Space::new().height(Length::Fixed(10.0)),

            // Glance Card: Trackers Blocked
            container(
                row![
                    column![
                        text("TRACKERS BLOCKED").size(9).color(Color::from_rgb(0.4, 0.5, 0.6)),
                        Space::new().height(Length::Fixed(2.0)),
                        text(format!("{}", state.trackers_blocked)).size(20).color(Color::from_rgb(0.0, 0.9, 0.5)),
                    ],
                    Space::new().width(Length::Fill),
                    button("Shield").padding(8).on_press(Message::ToggleShieldPanel)
                ]
            ).padding(14).width(Length::Fill).style(|_| container::Style {
                background: Some(Color::from_rgb(0.09, 0.11, 0.15).into()),
                border: Border { color: Color::from_rgb(0.18, 0.22, 0.3), width: 1.0, radius: 12.0.into() },
                ..Default::default()
            }),

            Space::new().height(Length::Fill),

            // --- THUMB ZONE (Bottom Third) ---
            column![
                // Pinned App Dock
                container(
                    row![
                        dock_icon("☎"), Space::new().width(Length::Fill),
                        dock_icon("💬"), Space::new().width(Length::Fill),
                        dock_icon("🌐"), Space::new().width(Length::Fill),
                        dock_icon("🖼"),
                    ]
                ).padding(12).width(Length::Fill).style(|_| container::Style {
                    background: Some(Color::from_rgb(0.07, 0.08, 0.11).into()),
                    border: Border { color: Color::from_rgb(0.15, 0.18, 0.25), width: 1.0, radius: 14.0.into() },
                    ..Default::default()
                }),

                Space::new().height(Length::Fixed(10.0)),

                // Search Bar Pill (Replaces App Grid)
                text_input("Search apps, files, settings", &state.search_query)
                    .on_input(Message::SearchChanged)
                    .padding(12),

                Space::new().height(Length::Fixed(10.0)),

                // Space Switcher Dots / Indicator Bar
                row![
                    Space::new().width(Length::Fill),
                    button(text("•  •  •").size(14).color(accent_color))
                        .padding(6)
                        .on_press(Message::ToggleSpacesPanel)
                        .style(|_, _| button::Style { background: None, ..Default::default() }),
                    Space::new().width(Length::Fill),
                ]
            ]
        ]
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .style(|_| container::Style {
            background: Some(Color::from_rgb(0.04, 0.05, 0.07).into()),
            text_color: Some(Color::WHITE),
            ..Default::default()
        })
        .into()
}

fn space_card<'a>(title: &'a str, subtitle: &'a str, is_active: bool, accent: Color, msg: Message) -> Element<'a, Message> {
    button(
        container(
            column![
                row![
                    text(title).size(15).color(Color::WHITE),
                    Space::new().width(Length::Fill),
                    text(if is_active { "Active" } else { "Switch" }).size(11).color(accent)
                ],
                Space::new().height(Length::Fixed(4.0)),
                text(subtitle).size(11).color(Color::from_rgb(0.6, 0.65, 0.75)),
            ]
        ).padding(12).width(Length::Fill)
    )
    .padding(0)
    .on_press(msg)
    .style(move |_, _| button::Style {
        background: Some(if is_active { Color::from_rgb(0.11, 0.14, 0.2).into() } else { Color::from_rgb(0.07, 0.08, 0.11).into() }),
        border: Border { color: if is_active { accent } else { Color::from_rgb(0.18, 0.22, 0.3) }, width: if is_active { 1.5 } else { 1.0 }, radius: 10.0.into() },
        ..Default::default()
    })
    .into()
}

fn dock_icon<'a>(symbol: &'a str) -> Element<'a, Message> {
    container(text(symbol).size(18))
        .width(44)
        .height(44)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color::from_rgb(0.12, 0.15, 0.2).into()),
            border: Border { radius: 12.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}