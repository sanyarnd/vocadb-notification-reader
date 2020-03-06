package vocadb.notification.reader

import com.tngtech.archunit.core.domain.JavaClasses
import com.tngtech.archunit.core.importer.ImportOption.DoNotIncludeTests
import com.tngtech.archunit.junit.AnalyzeClasses
import com.tngtech.archunit.junit.ArchTest
import com.tngtech.archunit.library.Architectures.layeredArchitecture
import com.tngtech.archunit.library.GeneralCodingRules

@AnalyzeClasses(packagesOf = [Application::class], importOptions = [DoNotIncludeTests::class])
class ArchitectureTest {
    @ArchTest
    fun `standard layer check`(importedClasses: JavaClasses) {
        val rule = layeredArchitecture()
            .layer("Configuration").definedBy("vocadb.notification.reader.configuration..")
            .layer("Controller").definedBy("vocadb.notification.reader.web..")
            .layer("Service").definedBy("vocadb.notification.reader.service..")

            .whereLayer("Controller").mayNotBeAccessedByAnyLayer()
            .whereLayer("Service").mayOnlyBeAccessedByLayers("Configuration", "Controller")

        rule.check(importedClasses)
    }

    @ArchTest
    fun `default general checks`(classes: JavaClasses) {
        GeneralCodingRules.NO_CLASSES_SHOULD_ACCESS_STANDARD_STREAMS.check(classes)
        GeneralCodingRules.NO_CLASSES_SHOULD_USE_JAVA_UTIL_LOGGING.check(classes)
        GeneralCodingRules.NO_CLASSES_SHOULD_USE_JODATIME.check(classes)
        GeneralCodingRules.NO_CLASSES_SHOULD_THROW_GENERIC_EXCEPTIONS.check(classes)
    }
}
