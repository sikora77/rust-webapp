#!/bin/bash
su -c systemctl start postgresql
su - postgres  -c "/usr/lib/postgresql/15/bin/pg_ctl -D /usr/local/pgsql/data -l /usr/local/pgsql_logs/logfile start"
backend