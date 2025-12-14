#[macro_export]
macro_rules! collect_model_field {
    // -------------------------
    // map + Option<T> -> T (default)
    // -------------------------
    (map, $items:expr, $field_fn:expr, $default:expr) => {{
        let collected: Vec<_> = $items
            .iter()
            .map(|item| match ($field_fn)(item).clone() {
                Some(v) => v,
                None => $default,
            })
            .collect();
        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // map + direct (non-Option) field
    // -------------------------
    (map, $items:expr, $field_fn:expr) => {{
        let collected: Vec<_> = $items.iter().map(|item| ($field_fn)(item)).collect();
        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // filter_map: closure must return Option<U>
    // -------------------------
    (filter_map, $items:expr, $field_fn:expr) => {{
        let collected: Vec<_> = $items.iter().filter_map(|item| ($field_fn)(item)).collect();
        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // Use when field_fn returns Option<&T> but owned T is needed:
    // -------------------------
    (filter_map_cloned, $items:expr, $field_fn:expr) => {{
        let collected: Vec<_> = $items
            .iter()
            .filter_map(|item| ($field_fn)(item).cloned())
            .collect();

        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // filter + then filter_map
    // -------------------------
    (filter_map, $items:expr, $pred:expr, $field_fn:expr) => {{
        let collected: Vec<_> = $items
            .iter()
            .filter(|item| ($pred)(item))
            .filter_map(|item| ($field_fn)(item))
            .collect();
        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // filter + then filter_map_cloned
    // -------------------------
    (filter_map_cloned, $items:expr, $pred:expr, $field_fn:expr) => {{
        let collected: Vec<_> = $items
            .iter()
            .filter(|item| ($pred)(item))
            .filter_map(|item| ($field_fn)(item).cloned())
            .collect();
        (!collected.is_empty()).then_some(collected)
    }};

    // -------------------------
    // filter + map + defaulting Option<T> -> T (default)
    // -------------------------
    (filter_map_default, $items:expr, $pred:expr, $field_fn:expr, $default:expr) => {{
        let collected: Vec<_> = $items
            .iter()
            .filter(|item| ($pred)(item))
            .map(|item| match ($field_fn)(item).clone() {
                Some(v) => v,
                None => $default,
            })
            .collect();
        (!collected.is_empty()).then_some(collected)
    }};
}
