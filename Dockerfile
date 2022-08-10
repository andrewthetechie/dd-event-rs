FROM scratch
ARG TARGETARCH

COPY dd-event-$TARGETARCH/dd-event /dd-event

CMD ["/dd-event"]
