use crate::tests::test_from_busybox_sed;

#[test]
fn replacement_with_capgroup() {
    test_from_busybox_sed("Season_01/S01E01", "S01E01", "s/S([0-9]+)E/Season_\\1\\/S\\1E/");
}