import Gtk

let status = Application.run {
    let window = ApplicationWindowRef(application: $0)
    window.title = "Hello, world"
    window.setDefaultSize(width: 320, height: 240)
    let label = LabelRef(str: "Hello, SwiftGtk")
    window.add(widget: label)
    window.showAll()
}
