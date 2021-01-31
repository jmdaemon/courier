#include <gtk/gtk.h>

#include "app.h"
#include "appwin.h"

struct _CourierApp { GtkApplication parent; };

G_DEFINE_TYPE(CourierApp, courier_app, GTK_TYPE_APPLICATION);

static void courier_app_init (CourierApp *app) {} 
static void courier_app_activate (GApplication *app) { 
  CourierAppWindow *win; win = courier_app_window_new (COURIER_APP (app));
  gtk_window_present (GTK_WINDOW (win));
}

static void courier_app_class_init (CourierAppClass *class) {
  G_APPLICATION_CLASS (class)->activate = courier_app_activate;
}

CourierApp * courier_app_new (void) {
  return g_object_new (APP_TYPE, "application-id", "org.gtk.exampleapp", "flags", G_APPLICATION_HANDLES_OPEN, NULL);
}
