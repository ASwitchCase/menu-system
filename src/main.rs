mod menu_system;
use menu_system::{MenuNode, MenuTree};

fn main() {
    let mut main_menu: MenuNode = MenuNode::new(String::from("Main menu"));
    let mut inventory_menu: MenuNode = MenuNode::new(String::from("Inventory"));
    let item_use_menu: MenuNode = MenuNode::new(String::from("Item Use"));
    let skills_menu: MenuNode = MenuNode::new(String::from("Skills"));
    let settings_menu: MenuNode = MenuNode::new(String::from("Settings"));

    inventory_menu.add_children(Box::new([item_use_menu]));

    main_menu.add_children(Box::new([inventory_menu, skills_menu, settings_menu]));

    let mut menu: MenuTree = MenuTree::new(main_menu);
    menu.next(1);
    println!(
        "{:#?} : {:#?} ",
        menu.current().name,
        menu.current().children
    );
}
