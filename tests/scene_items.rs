#![cfg(feature = "test-integration")]

use anyhow::Result;
use either::Either;
use obws::requests::{
    DuplicateSceneItem, SceneItemProperties, SceneItemRender, SceneItemSpecification,
};

use common::{TEST_SCENE, TEXT_SOURCE};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scene_items();

    client.get_scene_item_list(Some(TEST_SCENE)).await?;

    let props = client
        .get_scene_item_properties(Some(TEST_SCENE), Either::Left(TEXT_SOURCE))
        .await?;
    client
        .reset_scene_item(Some(TEST_SCENE), Either::Left(TEXT_SOURCE))
        .await?;
    client
        .set_scene_item_properties(SceneItemProperties {
            scene_name: Some(TEST_SCENE),
            item: Either::Left(TEXT_SOURCE),
            position: Some((&props.position).into()),
            rotation: Some(props.rotation),
            scale: Some((&props.scale).into()),
            crop: Some((&props.crop).into()),
            visible: Some(props.visible),
            locked: Some(props.locked),
            bounds: Some((&props.bounds).into()),
        })
        .await?;

    client
        .set_scene_item_render(SceneItemRender {
            scene_name: Some(TEST_SCENE),
            source: TEXT_SOURCE,
            item: None,
            render: !props.visible,
        })
        .await?;
    client
        .set_scene_item_render(SceneItemRender {
            scene_name: Some(TEST_SCENE),
            source: TEXT_SOURCE,
            item: None,
            render: props.visible,
        })
        .await?;

    let item = client
        .duplicate_scene_item(DuplicateSceneItem {
            from_scene: Some(TEST_SCENE),
            to_scene: Some(TEST_SCENE),
            item: SceneItemSpecification {
                id: None,
                name: Some(TEXT_SOURCE),
            },
        })
        .await?;
    client
        .delete_scene_item(
            Some(TEST_SCENE),
            SceneItemSpecification {
                id: Some(item.item.id),
                name: None,
            },
        )
        .await?;

    // TODO: Need to create a source first, but there is no way to delete it afterwards.
    // Therefore, we don't call this function until a method becomes available.
    //client.add_scene_item(AddSceneItem{});

    Ok(())
}
