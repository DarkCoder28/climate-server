FROM alpine:latest
EXPOSE 3000
#ENV WEB_ADDRESS="0.0.0.0"
#ENV WEB_PORT="3000"
#ENV SQL_HOST="mysql"
#ENV SQL_PORT="3306"
#ENV SQL_USER
#ENV SQL_PASS
#ENV SQL_DB

COPY climate-server /srv/climate-server
WORKDIR /srv/
RUN chown root:root ./climate-server ; chmod 555 ./climate-server
ENTRYPOINT /srv/climate-server