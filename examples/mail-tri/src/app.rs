use dq_components::{
    badge, dq_button, dq_list_header, dq_list_item, dq_section, dq_text_input,
    dq_toast, ButtonSize, ButtonVariant, ButtonWidth, StudioIcon, ToastVariant,
};
use dq_layout::dq_studio_nav;
use dq_theme::page_container;
use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Nav(NavId),
    TaskQueue,
    MailSelected(usize),
    SearchChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavId {
    Inbox,
    Sent,
    Drafts,
    Trash,
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
                id: "sent".into(),
                icon: StudioIcon::Video,
                label: "已发送".into(),
                message: Message::Nav(NavId::Sent),
            },
            dq_layout::StudioNavItem {
                id: "drafts".into(),
                icon: StudioIcon::Audio,
                label: "草稿".into(),
                message: Message::Nav(NavId::Drafts),
            },
            dq_layout::StudioNavItem {
                id: "trash".into(),
                icon: StudioIcon::Gallery,
                label: "回收站".into(),
                message: Message::Nav(NavId::Trash),
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
            Message::MailSelected(idx) => {
                println!("Selected mail {}", idx);
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
            NavId::Sent => "sent",
            NavId::Drafts => "drafts",
            NavId::Trash => "trash",
            NavId::Settings => "settings",
        };

        let brand = text("DanQing Mail")
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
        // Left: folder list (compact)
        let folders = column![
            dq_list_item("收件箱", Some("12 封未读"), None, true, false, None),
            dq_list_item("已发送", None, None, false, false, None),
            dq_list_item("草稿", Some("2 封"), None, false, false, None),
            dq_list_item("回收站", None, None, false, false, None),
        ]
        .spacing(0.0)
        .width(Length::Fill);

        let left_sidebar = container(folders)
            .padding([spacing::MD, 0.0])
            .width(Length::Fill)
            .height(Length::Fill);

        // Center: mail list
        let search = dq_text_input("搜索邮件…", &self.search, Message::SearchChanged);
        let mut mail_list = column![].spacing(0.0).width(Length::Fill);

        mail_list = mail_list.push(dq_list_header("今天"));
        mail_list = mail_list.push(dq_list_item(
            "Re: dq-ui v2 设计评审",
            Some("Karri: 色彩系统看起来不错"),
            Some("10:23"),
            false,
            true,
            Some(Message::MailSelected(0)),
        ));
        mail_list = mail_list.push(dq_list_item(
            "Weekly Sync 纪要",
            Some("你: 好的，已确认"),
            Some("09:45"),
            false,
            false,
            Some(Message::MailSelected(1)),
        ));

        mail_list = mail_list.push(dq_list_header("昨天"));
        mail_list = mail_list.push(dq_list_item(
            "Phase 2 排期",
            Some("Romain: Teams + Mail 示例已准备好"),
            Some("昨天"),
            false,
            false,
            Some(Message::MailSelected(2)),
        ));

        let center_list = column![
            container(search).padding([spacing::MD, spacing::LG]).width(Length::Fill),
            scrollable(mail_list).width(Length::Fill).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        // Right: reading pane
        let reading = column![
            row![
                text("Re: dq-ui v2 设计评审")
                    .size(typography::TITLE)
                    .color(color::TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                badge("未读"),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
            text("Karri Saarinen <karri@linear.app>")
                .size(typography::CAPTION)
                .color(color::TEXT_TERTIARY),
            container(
                column![
                    text("色彩系统看起来不错，特别是 LCH 的生成逻辑。建议把 contrast 变量的范围再测试一下，确保在 0.3 和 1.0 两端都有良好的可读性。")
                        .size(typography::BODY)
                        .color(color::TEXT_SECONDARY),
                    Space::new().height(spacing::MD),
                    text("另外，Panel 的去边框化效果比预期好，但是否需要在 hover 时显示微妙的边框？")
                        .size(typography::BODY)
                        .color(color::TEXT_SECONDARY),
                ]
                .spacing(spacing::SM)
                .width(Length::Fill),
            )
            .padding([spacing::MD, 0.0])
            .width(Length::Fill),
            row![
                dq_button(
                    "回复",
                    ButtonVariant::Secondary,
                    ButtonSize::Sm,
                    ButtonWidth::Hug,
                    None,
                ),
                dq_button(
                    "转发",
                    ButtonVariant::Ghost,
                    ButtonSize::Sm,
                    ButtonWidth::Hug,
                    None,
                ),
                dq_button(
                    "归档",
                    ButtonVariant::Ghost,
                    ButtonSize::Sm,
                    ButtonWidth::Hug,
                    None,
                ),
            ]
            .spacing(spacing::SM),
            Space::new().height(Length::Fill),
        ]
        .spacing(spacing::MD)
        .width(Length::Fill)
        .height(Length::Fill);

        dq_components::dq_split_view(
            left_sidebar.into(),
            center_list.into(),
            container(reading).padding([spacing::LG, spacing::XL]).into(),
            Length::Fixed(180.0),
            Length::FillPortion(2),
            Length::FillPortion(3),
        )
    }
}

fn placeholder_page(nav: NavId) -> Element<'static, Message> {
    let label = match nav {
        NavId::Sent => "已发送 — 即将推出",
        NavId::Drafts => "草稿 — 即将推出",
        NavId::Trash => "回收站 — 即将推出",
        NavId::Settings => "设置 — 即将推出",
        NavId::Inbox => unreachable!(),
    };
    container(text(label).color(color::TEXT_SECONDARY))
        .padding(32)
        .into()
}
