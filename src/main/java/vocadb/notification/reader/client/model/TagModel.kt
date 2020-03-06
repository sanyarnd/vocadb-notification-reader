package vocadb.notification.reader.client.model

import vocadb.notification.reader.client.model.enums.Language
import vocadb.notification.reader.client.model.enums.Status
import java.time.LocalDateTime

data class TagBaseContract constructor(
    val id: Long,
    val name: String,
    val categoryName: String? = null,
    val additionalNames: String? = null,
    val urlSlug: String? = null
)

data class TagUsageForApiContract constructor(
    val tag: TagBaseContract,
    val count: Long
)

data class TagForApiContract constructor(
    val additionalNames: String? = null,
    val aliasedTo: TagBaseContract? = null,
    val categoryName: String? = null,
    val createDate: LocalDateTime? = null,
    val defaultNameLanguage: Language? = null,
    val description: String? = null,
    val id: Int? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringWithIdContract>? = null,
    val parent: TagBaseContract? = null,
    val relatedTags: Collection<TagBaseContract>? = null,
    val status: Status? = null,
    val targets: Int? = null,
    val translatedDescription: EnglishTranslatedStringContract? = null,
    val urlSlug: String? = null,
    val usageCount: Int? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkForApiContract>? = null
)

data class LocalizedStringWithIdContract constructor(
    val id: Int? = null,
    val language: Language? = null,
    val value: String? = null
)

data class EnglishTranslatedStringContract constructor(
    val english: String? = null,
    val original: String? = null
)
