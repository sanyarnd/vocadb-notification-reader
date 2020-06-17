package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum FilterType {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("ArtistType")
    ARTIST_TYPE,
    @JsonProperty("WebLink")
    WEB_LINK,
    @JsonProperty("HasUserAccount")
    HAS_USER_ACCOUNT,
    @JsonProperty("RootVoicebank")
    ROOT_VOICEBANK,
    @JsonProperty("VoiceProvider")
    VOICE_PROVIDER,
    @JsonProperty("HasStoreLink")
    HAS_STORE_LINK,
    @JsonProperty("HasTracks")
    HAS_TRACKS,
    @JsonProperty("NoCoverPicture")
    NO_COVER_PICTURE,
    @JsonProperty("HasAlbum")
    HAS_ALBUM,
    @JsonProperty("HasOriginalMedia")
    HAS_ORIGINAL_MEDIA,
    @JsonProperty("HasMedia")
    HAS_MEDIA,
    @JsonProperty("HasMultipleVoicebanks")
    HAS_MULTIPLE_VOICEBANKS,
    @JsonProperty("HasPublishDate")
    HAS_PUBLISH_DATE,
    @JsonProperty("Lyrics")
    LYRICS,
    @JsonProperty("LyricsContent")
    LYRICS_CONTENT;
}
