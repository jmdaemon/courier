#ifndef __APPWIN_H
#define __APPWIN_H

#include <gtk/gtk.h>
#include "app.h"

#define APP_WINDOW_TYPE (courier_app_window_get_type ())
G_DECLARE_FINAL_TYPE (CourierAppWindow, courier_app_window, COURIER, APP_WINDOW, GtkApplicationWindow)

CourierAppWindow *courier_app_window_new (CourierApp *app);
void courier_app_window_open (CourierAppWindow *win, GFile *file);

#endif

