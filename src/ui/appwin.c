#include <gtk/gtk.h>

#include "app.h"
#include "appwin.h"

struct _CourierAppWindow { GtkApplicationWindow parent; };

G_DEFINE_TYPE(CourierAppWindow, courier_app_window, GTK_TYPE_APPLICATION_WINDOW);

static void courier_app_window_init (CourierAppWindow *app) { }
static void courier_app_window_class_init (CourierAppWindowClass *class) { }

CourierAppWindow * courier_app_window_new (CourierApp *app) {
  return g_object_new (APP_WINDOW_TYPE, "application", app, NULL);
}
