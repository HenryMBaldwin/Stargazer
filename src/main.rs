fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "Hello, world!";
            color: green;
        }
    }
}