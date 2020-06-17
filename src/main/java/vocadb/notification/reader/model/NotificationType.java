package vocadb.notification.reader.model;

public enum NotificationType {
    SONG,
    ARTIST,
    EVENT,
    ALBUM,
    REPORT,
    UNKNOWN;

    public static NotificationType of(String code) {
        switch (code) {
            case "S":
                return SONG;
            case "Ar":
                return ARTIST;
            case "E":
                return EVENT;
            case "Al":
                return ALBUM;
            case "R":
                return REPORT;
            default:
                return UNKNOWN;
        }
    }
}
