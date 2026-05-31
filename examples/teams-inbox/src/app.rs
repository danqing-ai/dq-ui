use dq_components::{
    badge, dq_button, dq_list_empty, dq_list_header, dq_list_item, dq_section, dq_text_input,
    ButtonSize, ButtonVariant, ButtonWidth, StudioIcon,
};
use dq_layout::dq_studio_nav;
use dq_theme::page_container;
use dq_tokens::{color, layout, spacing, typography};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Nav(NavId),
    TaskQueue,
    InboxItemSelected(usize),
    SearchChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavId {
    Inbox,
    Tasks,
    Calendar,
    Settings,
}

pub struct App {
    nav: NavId,
    search: String,
    nav_items: Vec<dq_layout::StudioNavItem<Message>>,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        let nav_items = vec![
            dq_layout::StudioNavItem {
                id: "inbox".into(),
                icon: StudioIcon::Image,
                label: "收件箱".into(),
                message: Message::Nav(NavId::Inbox),
            },
            dq_layout::StudioNavItem {
                id: "tasks".into(),
                icon: StudioIcon::Video,
                label: "任务".into(),
                message: Message::Nav(NavId::Tasks),
            },
            dq_layout::StudioNavItem {
                id: "calendar".into(),
                icon: StudioIcon::Audio,
                label: "日历".into(),
                message: Message::Nav(NavId::Calendar),
            },
            dq_layout::StudioNavItem {
                id: "settings".into(),
                icon: StudioIcon::Settings,
                label: "设置".into(),
                message: Message::Nav(NavId::Settings),
            },
        ];

        (
            Self {
                nav: NavId::Inbox,
                search: String::new(),
                nav_items,
            },
            iced::Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Nav(id) => {
                self.nav = id;
                iced::Task::none()
            }
            Message::TaskQueue => iced::Task::none(),
            Message::InboxItemSelected(idx) => {
                println!("Selected inbox item {}", idx);
                iced::Task::none()
            }
            Message::SearchChanged(s) => {
                self.search = s;
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let active_id = match self.nav {
            NavId::Inbox => "inbox",
            NavId::Tasks => "tasks",
            NavId::Calendar => "calendar",
            NavId::Settings => "settings",
        };

        let brand = text("DanQing Teams")
            .size(typography::BODY)
            .color(color::TEXT_PRIMARY);

        let studio_nav = dq_studio_nav(
            brand.into(),
            &self.nav_items,
            active_id,
            "通知",
            Message::TaskQueue,
        );

        let body: Element<Message> = match self.nav {
            NavId::Inbox => self.inbox_view(),
            _ => placeholder_page(self.nav),
        };

        container(
            column![studio_nav, body]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(page_container)
        .into()
    }

    fn inbox_view(&self) -> Element<'_, Message> {
        let search = dq_text_input("搜索消息…", &self.search, Message::SearchChanged);

        let mut list = column![].spacing(0.0).width(Length::Fill);

        list = list.push(dq_list_header("今天"));
        list = list.push(dq_list_item(
            "UI 组件库设计评审",
            Some("Karri: 滑动条的交互需要再细化一下"),
            Some("10:23"),
            false,
            true,
            Some(Message::InboxItemSelected(0)),
        ));
        list = list.push(dq_list_item(
            "Phase 2 排期确认",
            Some("你: 好的，下周一开始"),
            Some("09:45"),
            false,
            false,
            Some(Message::InboxItemSelected(1)),
        ));

        list = list.push(dq_list_header("昨天"));
        list = list.push(dq_list_item(
            "深色主题配色方案",
            Some("Andreas: LCH 色彩空间已验证"),
            Some("昨天"),
            false,
            false,
            Some(Message::InboxItemSelected(2)),
        ));
        list = list.push(dq_list_item(
            "Rust workspace 迁移",
            Some("Romain: 已完成初步拆分"),
            Some("昨天"),
            true,
            false,
            Some(Message::InboxItemSelected(3)),
        ));

        let sidebar = column![
            container(search).padding([spacing::MD, spacing::LG]).width(Length::Fill),
            scrollable(list).width(Length::Fill).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let detail = column![
            dq_section(
                dq_components::SectionIcon::Document,
                "UI 组件库设计评审",
                None,
                Some(
                    column![
                        text("Karri Saarinen")
                            .size(typography::BODY)
                            .color(color::TEXT_PRIMARY),
                        text("滑动条的交互需要再细化一下，特别是拖拽时的反馈。另外，Focus ring 的颜色在高对比度模式下需要调整。")
                            .size(typography::BODY)
                            .color(color::TEXT_SECONDARY),
                        row![
                            dq_button(
                                "回复",
                                ButtonVariant::Secondary,
                                ButtonSize::Sm,
                                ButtonWidth::Hug,
                                None,
                            ),
                            dq_button(
                                "标记已读",
                                ButtonVariant::Ghost,
                                ButtonSize::Sm,
                                ButtonWidth::Hug,
                                None,
                            ),
                        ]
                        .spacing(spacing::SM),
                    ]
                    .spacing(spacing::MD)
                    .width(Length::Fill)
                    .into(),
                ),
            ),
            Space::new().height(Length::Fill),
        ]
        .spacing(spacing::MD)
        .width(Length::Fill)
        .height(Length::Fill);

        dq_components::dq_split_view_two(
            sidebar.into(),
            container(detail).padding([spacing::LG, spacing::XL]).into(),
            Length::FillPortion(2),
        )
    }
}

fn placeholder_page(nav: NavId) -> Element<'static, Message> {
    let label = match nav {
        NavId::Tasks => "任务 — 即将推出",
        NavId::Calendar => "日历 — 即将推出",
        NavId::Settings => "设置 — 即将推出",
        NavId::Inbox => unreachable!(),
    };
    container(text(label).color(color::TEXT_SECONDARY))
        .padding(32)
        .into()
}
