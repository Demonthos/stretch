#[test]
fn wrap_row() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(31f32),
                    height: stretch2::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(32f32),
                    height: stretch2::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(33f32),
                    height: stretch2::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(34f32),
                    height: stretch2::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_wrap: stretch2::style::FlexWrap::Wrap,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 60f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 31f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 32f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 31f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 33f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 63f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node3).unwrap().size.width, 34f32);
    assert_eq!(stretch.layout(node3).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node3).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node3).unwrap().location.y, 30f32);
}
