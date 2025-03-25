use std::collections::HashMap;
use std::env;

// CRSスクリプトの文字列をコンパイル時定数として定義する。
const USE_UTF8: &str = r#"encoding "utf-8;"#;
const FORM: &str =
"Form Form1 {
	Width = 390;
	Height = 800;
";
const END_BLOCK: &str = "}";

const OBJECT_X: &str = "X = 10;";
const OBJECT_WIDTH: &str = "Width = ^.Width - 20;";
const OBJECT_HEIGHT: &str = "Height = 30;";
const ONTOUCH: &str = "Function OnTouch() {}";
const DESIGNTIME: &str = r#"
	if (!$DESIGNTIME) {

	}
"#;
const ERR: &str = r#"
	Function err(e) {
		MessageBox(e.Message, e.Method + "-" + str(e.Code));
	}
"#;

fn object_y(total_count: &u32) -> String {
	const MARGIN: u32 = 10;
	const HEIGHT: u32 = 30;
	format!("Y = {};", MARGIN + (HEIGHT + MARGIN) * total_count)
}

fn make_common_part(name: &str, this_count: &u32, total_count: &u32) -> String {
	format!("
	{} {}{} {{
		{}
		{}
		{}
		{}
",
	name, name, this_count,
	OBJECT_X,
	object_y(total_count),
	OBJECT_WIDTH,
	OBJECT_HEIGHT)
}

fn make_common_object(name: &str, this_count: &u32, total_count: &u32) -> String {
	format!("{}
	}}", make_common_part(name, this_count, total_count))
}

fn make_button(name: &str, this_count: &u32, total_count: &u32) -> String {
	format!("{}
		{}
	}}",
	make_common_part(name, this_count, total_count), ONTOUCH)
}

fn make_items(name: &str) -> String {
	const ITEMS: &str = r#" items[3];
		items[0] = { title = "item1"; value = 0; }
		items[1] = { title = "item2"; value = 1; }
		items[2] = { title = "item3"; value = 2; }
"#;
	let item_name = match name {
		"ListBox" => "ListItem",
		"CheckBox" => "CheckItem",
		"OptionButton" => "OptionItem",
		"PulldownList" => "PulldownItem",
		"SwitchButton" => "SwitchItem",
		_ => ""
	};
	format!("		{}{}", item_name, ITEMS)
}

fn make_list(name: &str, this_count: &u32, total_count: &u32) -> String {
	format!("{}{}
	}}",
	make_common_part(name, this_count, total_count), make_items(name))
}

fn main() {
	let object_table = HashMap::from([
		("Button", make_button as fn(&str, &u32, &u32) -> String),
		("ImageButton", make_button),
		("Label", make_common_object),
		("ImageLabel", make_common_object),
		("EditBox", make_common_object),
		("TextBox", make_common_object),
		("DateEdit", make_common_object),
		("NumberEdit", make_common_object),
		("PulldownList", make_list),
		("CheckBox", make_list),
		("OptionButton", make_list),
		("ListBox", make_list),
		("SwitchButton", make_list),
		("CanvasView", make_common_object),
		("HtmlView", make_common_object),
		("CameraCaptureView", make_common_object),
		("ExpandableList", make_common_object),
		("ImageFilter", make_common_object),
		("SlideMenu", make_common_object),
		("MaskEdit", make_common_object),
		("MediaPlayer", make_common_object),
		("MediaRecorder", make_common_object),
	]);

	let mut count_by_objects: HashMap<&str, u32> = HashMap::new();
	let mut total_count = 0u32;

	let mut result = format!("{}\n\n{}{}", USE_UTF8, FORM, ERR);
	let args: Vec<String> = env::args().collect();
	for arg in args {
		for (&name, &func) in &object_table {
			if !arg.eq_ignore_ascii_case(name) {
				continue;
			}
			*count_by_objects.entry(name).or_insert(0) += 1;
			result += &func(name, count_by_objects.get(name).unwrap(), &total_count);
			total_count += 1;
		}
	}
	result = format!("{}\n{}{}", result, DESIGNTIME, END_BLOCK);
	println!("{}", result);
}
