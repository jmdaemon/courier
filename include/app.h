#ifndef __MY_APP_H
#define __MY_APP_H

#include <gtk/gtk.h>

#define APP_TYPE (courier_app_get_type ())
G_DECLARE_FINAL_TYPE (CourierApp, courier_app, COURIER, APP, GtkApplication)
CourierApp *courier_app_new (void);

#endif 

