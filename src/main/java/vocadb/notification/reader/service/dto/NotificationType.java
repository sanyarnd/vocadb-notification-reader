package vocadb.notification.reader.service.dto;

public enum NotificationType {
    SONG,
    ARTIST,
    EVENT,
    ALBUM,
    REPORT,
    UNKNOWN;

    public static NotificationType of(String code) {
        NotificationType result;
        switch (code) {
            case "S":
                result = SONG;
                break;
            case "Ar":
                result = ARTIST;
                break;
            case "E":
                result = EVENT;
                break;
            case "Al":
                result = ALBUM;
                break;
            case "R":
                result = REPORT;
                break;
            default:
                result = UNKNOWN;
                break;
        }
        return result;
    }
}
