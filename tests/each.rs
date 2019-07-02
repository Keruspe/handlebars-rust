use handlebars::Handlebars;
use serde_json::json;

#[test]
fn test_nested_each() {
    let hbs = Handlebars::new();

    let data = json!({
        "classes": [
            {
                "methods": [
                    {"id": 1},
                    {"id": 2}
                ]
            },
            {
                "methods": [
                    {"id": 3},
                    {"id": 4}
                ]
            },
        ],
    });

    let template = "{{#*inline \"test\"}}{{method.id}};{{/inline}}{{#each classes as |class|}}{{#each class.methods as |method|}}{{> test method}}{{/each}}{{/each}}";
    assert_eq!(hbs.render_template(template, &data).unwrap(), "1;2;3;4;");
}
