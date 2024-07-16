use super::book::*;


// pub fn book_card(&self, book: &Book, size: Size) -> Element<Message> {
//     let spacing = theme::active().cosmic().spacing;

//     let mut card_content = cosmic::widget::column()
//         .spacing(spacing.space_xxxs)
//         .width(Length::Fill);
//     // .height(Length::Shrink);

//     let handle = match self.book_covers.get(&book.name) {
//         Some(h) => h.clone(),
//         None => cosmic::widget::image::Handle::from_path("res/covers/rr-image.png"),
//     };
//     dbg!(&handle);
//     dbg!(&book);

//     card_content = card_content.push(
//         cosmic::iced::widget::image(handle.clone())
//             .content_fit(cosmic::iced::ContentFit::Contain)
//             .width(Length::Fill)
//             .border_radius([spacing.space_xxxs as f32; 4]),
//     );

//     card_content = card_content.push(
//         cosmic::widget::text(book.name.clone()).height(Length::Fixed(spacing.space_xl as f32)),
//     );

//     let card = container(card_content)
//         .padding(spacing.space_xxs)
//         .width(Length::Fixed(size.width))
//         .height(Length::Shrink)
//         .style(cosmic::theme::Container::Secondary);
//     // let mouse_area =
//     card.into()
// }
