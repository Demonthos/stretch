#[test]
fn absolute_layout_within_border() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                position_type: stretch2::style::PositionType::Absolute,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(0f32),
                    top: stretch2::style::Dimension::Points(0f32),
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
                position_type: stretch2::style::PositionType::Absolute,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    end: stretch2::style::Dimension::Points(0f32),
                    bottom: stretch2::style::Dimension::Points(0f32),
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
                position_type: stretch2::style::PositionType::Absolute,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(0f32),
                    top: stretch2::style::Dimension::Points(0f32),
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
                position_type: stretch2::style::PositionType::Absolute,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: stretch2::geometry::Rect {
                    end: stretch2::style::Dimension::Points(0f32),
                    bottom: stretch2::style::Dimension::Points(0f32),
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
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                padding: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                border: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(10f32),
                    end: stretch2::style::Dimension::Points(10f32),
                    top: stretch2::style::Dimension::Points(10f32),
                    bottom: stretch2::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 10f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 40f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 40f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 20f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 20f32);
    assert_eq!(stretch.layout(node3).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node3).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node3).unwrap().location.x, 30f32);
    assert_eq!(stretch.layout(node3).unwrap().location.y, 30f32);
}
