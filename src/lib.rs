use mlua::{Lua, ExternalError, prelude::LuaResult, Table};
use scraper::{Html, Selector, Node};
use std::borrow::Cow;

/// Generate a CSS selector pattern for **custom** Element in the HTML input that has attributes.
fn css_selector(_: &Lua, html: String) -> LuaResult<Cow<str>> {
    // Initialize a string to hold the resulting CSS selector patterns.
    let mut selectors = String::new();
    // Parse the input HTML into an HTML document tree.
    let document = Html::parse_document(&html);


    // Traverse the document tree and generate a selector pattern for each element with attributes.
    for node in document.tree.into_iter() {
        // Check if the node is an element.
        if let Node::Element(element) = node {
            // Initialize a string to hold the selector pattern for this element.
            let mut selector = element.name().to_string();

            // Add an attribute selector for each attribute of this element.
            for attr in element.attrs.iter() {
                // If the attribute value is not empty, add a CSS attribute selector for it.
                if !attr.1.to_string().is_empty() {
                    selector.push_str(&format!(
                        r#"[{}="{}"]"#,
                        attr.0.local,
                        attr.1.replace('\'', "\\'").replace('\"', "\\\"")
                    ));
                } 
                // Otherwise, add a CSS attribute selector for the attribute name only.
                else {
                    selector.push_str(&format!(r#"[{}]"#, attr.0.local));
                }
            }

            // If the element has at least one attribute, add the selector pattern to the result string.
            if selector.contains('[') {
                selectors.push_str(&selector);
            }
        }
    }

    Ok(Cow::from(selectors))
}

/// Given an HTML code string and a CSS selector, returns a vector of Cow strings
/// representing the HTML elements that match the CSS selector.
pub fn html_search(_: &Lua, (html_code, css_selector): (String, String)) -> Result<Vec<Cow<str>>, mlua::Error>{
    // Parse the HTML code string into an HTML document using the `Html::parse_document` method.
    let parsed_html = Html::parse_document(&html_code);
    
    // Parse the CSS selector string into a `Selector` using the `Selector::parse` method.
    let selector = Selector::parse(&css_selector);
    
    // If parsing the CSS selector succeeded, search the HTML document for elements matching the selector.
    let selected = match selector {
        Ok(s) => {
            let css_search_results = parsed_html.select(&s);
            let mut finds = Vec::new();
            
            // For each matched element, push a Cow string representing the element's HTML code
            // onto the `finds` vector.
            css_search_results.for_each(|x| {
                finds.push(Cow::from(x.html()));
            });
            Ok(finds)
        },
        
        // If parsing the CSS selector failed, return an error.
        Err(err) => {
            Err(err.to_string().to_lua_err())
        }
    };
    
    // Return the vector of Cow strings representing the matched HTML elements,
    // or an error if parsing the CSS selector failed.
    selected
}

#[mlua::lua_module]
fn lhtmlr(lua: &Lua) -> LuaResult<Table>{
    let html_search = lua.create_function(html_search)?;
    let css_selector = lua.create_function(css_selector)?;
    let table = lua.create_table().unwrap();
    table.set("html_search", html_search)?;
    table.set("css_selector", css_selector)?;
    Ok(table)
}
