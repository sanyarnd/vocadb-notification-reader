package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue

data class WebLinkContract constructor(
    val category: WebLinkCategory? = null,
    val description: String? = null,
    val descriptionOrUrl: String? = null,
    val id: Int? = null,
    val url: String? = null
)

data class WebLinkForApiContract constructor(
    val category: WebLinkCategory? = null,
    val description: String? = null,
    val id: Int? = null,
    val url: String? = null
)

enum class WebLinkCategory(@JsonValue val value: String) {
    OFFICIAL("Official"),
    COMMERCIAL("Commercial"),
    REFERENCE("Reference"),
    OTHER("Other");
}
