FROM nhl-app:latest AS nhl_app

FROM nginx:1.15-alpine

WORKDIR /app/public

COPY ./nhl.conf /etc/nginx/conf.d/default.conf

COPY --from=nhl_app ./app/public/* ./

RUN chown -R nginx /app