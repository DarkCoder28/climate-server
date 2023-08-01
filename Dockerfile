FROM alpine:latest
EXPOSE 3000
#ENV WEB_ADDRESS="0.0.0.0"
#ENV WEB_PORT="3000"
#ENV SQL_HOST="mysql"
#ENV SQL_PORT="3306"
#ENV SQL_USER
#ENV SQL_PASS
#ENV SQL_DB

COPY --chown=root:root --chmod=555 climate-server /srv/climate-server
WORKDIR /srv/
ENTRYPOINT /srv/climate-server