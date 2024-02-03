FROM nginx:1.25-alpine

COPY static /srv/static

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 7000
