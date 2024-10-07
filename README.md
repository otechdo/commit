<!-- TOC -->
* [Installation](#installation)
  * [Help](#help)
  * [Templates](#templates)
    * [List](#list)
    * [Add](#add)
    * [Remove](#remove)
    * [Available by default](#available-by-default)
    * [Get templates](#get-templates)
  * [Usage](#usage)
    * [For alerting-update](#for-alerting-update)
      * [Run](#run)
      * [Output](#output)
    * [For api-change](#for-api-change)
      * [Run](#run-1)
      * [Output](#output-1)
    * [For api-documentation](#for-api-documentation)
      * [Run](#run-2)
      * [Output](#output-2)
    * [For api](#for-api)
      * [Run](#run-3)
      * [Output](#output-3)
    * [For audit](#for-audit)
      * [Run](#run-4)
      * [Output](#output-4)
    * [For backup-creation](#for-backup-creation)
      * [Run](#run-5)
      * [Output](#output-5)
    * [For build](#for-build)
      * [Run](#run-6)
      * [Output](#output-6)
    * [For cache-update](#for-cache-update)
      * [Run](#run-7)
      * [Output](#output-7)
    * [For cherry-pick](#for-cherry-pick)
      * [Run](#run-8)
      * [Output](#output-8)
    * [For chore](#for-chore)
      * [Run](#run-9)
      * [Output](#output-9)
    * [For ci](#for-ci)
      * [Run](#run-10)
      * [Output](#output-10)
    * [For cleanup](#for-cleanup)
      * [Run](#run-11)
      * [Output](#output-11)
    * [For compliance](#for-compliance)
      * [Run](#run-12)
      * [Output](#output-12)
    * [For config](#for-config)
      * [Run](#run-13)
      * [Output](#output-13)
    * [For database-migration](#for-database-migration)
      * [Run](#run-14)
      * [Output](#output-14)
    * [For dependency-removal](#for-dependency-removal)
      * [Run](#run-15)
      * [Output](#output-15)
    * [For dependency-update](#for-dependency-update)
      * [Run](#run-16)
      * [Output](#output-16)
    * [For deployment](#for-deployment)
      * [Run](#run-17)
      * [Output](#output-17)
    * [For deprecate](#for-deprecate)
      * [Run](#run-18)
      * [Output](#output-18)
    * [For deprecation-notice](#for-deprecation-notice)
      * [Run](#run-19)
      * [Output](#output-19)
    * [For docs](#for-docs)
      * [Run](#run-20)
      * [Output](#output-20)
    * [For experiment](#for-experiment)
      * [Run](#run-21)
      * [Output](#output-21)
    * [For feat](#for-feat)
      * [Run](#run-22)
      * [Output](#output-22)
    * [For feature-addition](#for-feature-addition)
      * [Run](#run-23)
      * [Output](#output-23)
    * [For feature-toggle](#for-feature-toggle)
      * [Run](#run-24)
      * [Output](#output-24)
    * [For fix](#for-fix)
      * [Run](#run-25)
      * [Output](#output-25)
    * [For hotfix](#for-hotfix)
      * [Run](#run-26)
      * [Output](#output-26)
    * [For hotpatch](#for-hotpatch)
      * [Run](#run-27)
      * [Output](#output-27)
    * [For log-rotation-update](#for-log-rotation-update)
      * [Run](#run-28)
      * [Output](#output-28)
    * [For log-update](#for-log-update)
      * [Run](#run-29)
      * [Output](#output-29)
    * [For merge](#for-merge)
      * [Run](#run-30)
      * [Output](#output-30)
    * [For migration](#for-migration)
      * [Run](#run-31)
      * [Output](#output-31)
    * [For minor-update](#for-minor-update)
      * [Run](#run-32)
      * [Output](#output-32)
    * [For monitoring-update](#for-monitoring-update)
      * [Run](#run-33)
      * [Output](#output-33)
    * [For optimize](#for-optimize)
      * [Run](#run-34)
      * [Output](#output-34)
    * [For perf](#for-perf)
      * [Run](#run-35)
      * [Output](#output-35)
    * [For refactor](#for-refactor)
      * [Run](#run-36)
      * [Output](#output-36)
    * [For remove](#for-remove)
      * [Run](#run-37)
      * [Output](#output-37)
    * [For revert](#for-revert)
      * [Run](#run-38)
      * [Output](#output-38)
    * [For rollback-deployment](#for-rollback-deployment)
      * [Run](#run-39)
      * [Output](#output-39)
    * [For rollback](#for-rollback)
      * [Run](#run-40)
      * [Output](#output-40)
    * [For schema-update](#for-schema-update)
      * [Run](#run-41)
      * [Output](#output-41)
    * [For secret-update](#for-secret-update)
      * [Run](#run-42)
      * [Output](#output-42)
    * [For security-patch](#for-security-patch)
      * [Run](#run-43)
      * [Output](#output-43)
    * [For security](#for-security)
      * [Run](#run-44)
      * [Output](#output-44)
    * [For services](#for-services)
      * [Run](#run-45)
      * [Output](#output-45)
    * [For style](#for-style)
      * [Run](#run-46)
      * [Output](#output-46)
    * [For test](#for-test)
      * [Run](#run-47)
      * [Output](#output-47)
    * [For testing-environment-update](#for-testing-environment-update)
      * [Run](#run-48)
      * [Output](#output-48)
    * [For update](#for-update)
      * [Run](#run-49)
      * [Output](#output-49)
    * [For version-bump](#for-version-bump)
      * [Run](#run-50)
      * [Output](#output-50)
<!-- TOC -->

# Installation

> For the moment only feature cli is implemented

```bash
cargo install commiter --features git --features cli
```

> Supported : git mercurial fossil pijul

## Help

```bash
commiter --help
```

## Templates

### List

```bash
commiter template list
```

### Add

```bash
commiter template add
```

### Remove

```bash
commiter template remove -t template
```

### Available by default

```text
templates
├── alerting-update.txt
├── api-change.txt
├── api-documentation.txt
├── api.txt
├── audit.txt
├── backup-creation.txt
├── build.txt
├── cache-update.txt
├── cherry-pick.txt
├── chore.txt
├── ci.txt
├── cleanup.txt
├── compliance.txt
├── config.txt
├── database-migration.txt
├── dependency-removal.txt
├── dependency-update.txt
├── deployment.txt
├── deprecate.txt
├── deprecation-notice.txt
├── docs.txt
├── experiment.txt
├── feat.txt
├── feature-addition.txt
├── feature-toggle.txt
├── fix.txt
├── hotfix.txt
├── hotpatch.txt
├── log-rotation-update.txt
├── log-update.txt
├── merge.txt
├── migration.txt
├── minor-update.txt
├── monitoring-update.txt
├── optimize.txt
├── perf.txt
├── refactor.txt
├── remove.txt
├── revert.txt
├── rollback-deployment.txt
├── rollback.txt
├── schema-update.txt
├── secret-update.txt
├── security-patch.txt
├── security.txt
├── services.txt
├── squash.txt 
├── style.txt
├── testing-environment-update.txt
├── test.txt
├── update.txt
└── version-bump.txt

1 directory, 52 files
```

### Get templates

```bash
git clone https://github.com/otechdo/commit /tmp/commit && cp -rv /tmp/commit/templates $HOME/templates
```

## Usage

require manual track files

### For alerting-update

#### Run

```bash
commiter commit -t alerting-update
```

#### Output

```text
ALERTING UPDATE: {{ commit_message }}

Details:

    * Alerts added/modified for: {{ system_or_service }}.
    * New thresholds: {{ new_thresholds }}.
    * Notifications sent to: {{ notification_channels }}.
    * Reason for update: {{ reason_for_update }}.

Impact:

    * Incident detection improved.
    * Response time improved: {{ response_time_improvement }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For api-change

#### Run

```bash
commiter commit -t api-change
```

#### Output

```text
API CHANGE: {{ commit_message }}

Details:

    * API endpoint changed: {{ endpoint }}.
    * Reason for change: {{ reason_for_change }}.
    * Breaking changes: {{ breaking_changes }}.
    * New parameters: {{ new_parameters }}.
    * Deprecated endpoints: {{ deprecated_endpoints }}.

Impact:

    * API consumers impact: {{ consumer_impact }}.
    * Response time improvement: {{ response_time_improvement }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For api-documentation

#### Run

```bash
commiter commit -t api-documentation
```

#### Output

```text
API DOCUMENTATION: {{ commit_message }}

Details:

    * Documentation updated for: {{ api_endpoint }}.
    * Changes applied: {{ changes_applied }}.
    * New sections added: {{ new_sections }}.

Impact:

    * Documentation completeness improved.
    * API consumer understanding enhanced.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For api

#### Run

```bash
commiter commit -t api
```

#### Output

```text
API CHANGE: {{ commit_message }}

Details:

    * API endpoint affected: {{ endpoint }}.
    * Change type: {{ change_type }} (Addition, Modification, Removal).
    * Breaking changes: {{ breaking_changes }}.
    * New parameters: {{ new_parameters }}.
    * Deprecated endpoints: {{ deprecated_endpoints }}.

Impact:

    * Impact on API consumers: {{ consumer_impact }}.
    * Documentation updated: {{ documentation_update }}.

Author: {{ author }}
Date: {{ commit_date }}

Parent Commit: {{ parent_commit }}
```

### For audit

#### Run

```bash
commiter commit -t audit
```

#### Output

```text
AUDIT: {{ commit_message }}

Details:

    * Audit conducted on: {{ system_or_service }}.
    * Security vulnerabilities found: {{ vulnerabilities_found }}.
    * Recommended actions: {{ recommended_actions }}.

Impact:

    * System security posture improved.
    * Compliance status: {{ compliance_status }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For backup-creation

#### Run

```bash
commiter commit -t backup-creation
```

#### Output

```text
BACKUP CREATION: {{ commit_message }}

Details:

    * Backup created for: {{ system_or_service }}.
    * Data size: {{ data_size }}.
    * Storage location: {{ storage_location }}.

Impact:

    * Backup ensures data integrity and recovery options.
    * Backup time: {{ backup_time }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For build

#### Run

```bash
commiter commit -t build
```

#### Output

```text
BUILD: {{ commit_message }}

Dependency update:

    * Updated dependencies: {{ dependencies_updated }}.
    * Reason for update: {{ reason_for_update }}.
    * Version change: {{ version_change }}.

Impact:

    * Build stability and performance improved.
    * Issues resolved: {{ issues_resolved }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For cache-update

#### Run

```bash
commiter commit -t cache-update
```

#### Output

```text
CACHE UPDATE: {{ commit_message }}

Details:

    * Cache updated for: {{ system_or_service }}.
    * Cache expiry set to: {{ cache_expiry }}.
    * Cache invalidated for: {{ invalidated_items }}.

Impact:

    * Improved system performance and data freshness.
    * Response time improvement: {{ response_time_improvement }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For cherry-pick

#### Run

```bash
commiter commit -t cherry-pick
```

#### Output

```text
CHERRY PICK: {{ commit_message }}

Details:

    * Commit cherry-picked from: {{ source_branch }}.
    * Reason for cherry-picking: {{ reason_for_cherry_picking }}.

Impact:

    * Feature/bugfix applied to: {{ target_branch }}.
    * Conflicts resolved: {{ conflicts_resolved }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For chore

#### Run

```bash
commiter commit -t chore
```

#### Output

```text
CHORE: {{ commit_message }}

Details:

    * Routine task performed: {{ task }}.
    * System affected: {{ system_or_service }}.

Impact:

    * Codebase maintenance or improvement.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For ci

#### Run

```bash
commiter commit -t ci
```

#### Output

```text
CI UPDATE: {{ commit_message }}

Details:

    * CI pipeline changes: {{ ci_changes }}.
    * CI tool/version updated: {{ ci_tool_version }}.
    * CI task added/modified: {{ ci_task }}.

Impact:

    * Build automation improved.
    * Test coverage impact: {{ test_coverage_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For cleanup

#### Run

```bash
commiter commit -t cleanup
```

#### Output

```text
CLEANUP: {{ commit_message }}

Details:

    * Cleanup performed on: {{ system_or_service }}.
    * Files/Resources removed: {{ resources_removed }}.

Impact:

    * Codebase or system efficiency improved.
    * Unnecessary resources removed.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For compliance

#### Run

```bash
commiter commit -t compliance
```

#### Output

```text
COMPLIANCE: {{ commit_message }}

Details:

    * Compliance rule applied: {{ compliance_rule }}.
    * System/Module affected: {{ system_or_module }}.
    * Compliance standard: {{ compliance_standard }}.
    * Audit trail updated: {{ audit_trail_updated }}.

Impact:

    * Compliance with {{ compliance_standard }} ensured.
    * Legal and regulatory risks reduced.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For config

#### Run

```bash
commiter commit -t config
```

#### Output

```text
CONFIG: {{ commit_message }}

Details:

    * Configuration changed for: {{ system_or_service }}.
    * New configuration values: {{ new_configuration }}.
    * Previous configuration values: {{ previous_configuration }}.

Impact:

    * Configuration changes ensure: {{ configuration_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For database-migration

#### Run

```bash
commiter commit -t database-migration
```

#### Output

```text
DATABASE MIGRATION: {{ commit_message }}

Details:

    * Migration type: {{ migration_type }}.
    * Affected tables: {{ affected_tables }}.
    * Data integrity check: {{ data_integrity_check }}.
    * Downtime required: {{ downtime_required }}.

Impact:

    * Data migration impact: {{ data_migration_impact }}.
    * Performance impact: {{ performance_impact }}.
    * Backup status: {{ backup_status }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For dependency-removal

#### Run

```bash
commiter commit -t dependency-removal
```

#### Output

```text
DEPENDENCY REMOVAL: {{ commit_message }}

Details:

    * Removed dependencies: {{ removed_dependencies }}.
    * Reason for removal: {{ reason_for_removal }}.
    * Affected modules: {{ affected_modules }}.

Impact:

    * Codebase size reduced.
    * Build time improvements: {{ build_time_improvements }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For dependency-update

#### Run

```bash
commiter commit -t dependency-update
```

#### Output

```text
DEPENDENCY UPDATE: {{ commit_message }}

Details:

    * Updated dependencies: {{ updated_dependencies }}.
    * Reason for update: {{ reason_for_update }}.
    * Previous version: {{ previous_version }}.
    * Updated version: {{ updated_version }}.

Impact:

    * Performance and security improvements.
    * Issues resolved: {{ issues_resolved }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For deployment

#### Run

```bash
commiter commit -t deployment
```

#### Output

```text
DEPLOYMENT: {{ commit_message }}

Details:

    * Deployed version: {{ deployed_version }}.
    * Environment: {{ environment }} (e.g., production, staging).
    * Downtime duration: {{ downtime_duration }}.

Impact:

    * System/service availability: {{ availability_status }}.
    * Issues during deployment: {{ deployment_issues }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For deprecate

#### Run

```bash
commiter commit -t deprecate
```

#### Output

```text
DEPRECATE: {{ commit_message }}

Details:

    * Deprecated feature: {{ deprecated_feature }}.
    * Replacement feature: {{ replacement_feature }}.
    * Reason for deprecation: {{ reason_for_deprecation }}.

Impact:

    * Users need to migrate to: {{ replacement_feature }}.
    * Migration guide available: {{ migration_guide }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For deprecation-notice

#### Run

```bash
commiter commit -t deprecation-notice
```

#### Output

```text
DEPRECATION NOTICE: {{ commit_message }}

Details:

    * Feature to be deprecated: {{ feature_to_deprecate }}.
    * Deprecation timeline: {{ deprecation_timeline }}.
    * Replacement feature: {{ replacement_feature }}.

Impact:

    * User migration required.
    * End of life date: {{ eol_date }}.
    * Documentation updated: {{ documentation_update }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For docs

#### Run

```bash
commiter commit -t docs
```

#### Output

```text
DOCUMENTATION UPDATE: {{ commit_message }}

Details:

    * Sections updated: {{ updated_sections }}.
    * New sections added: {{ new_sections }}.
    * Technical changes documented: {{ changes_documented }}.

Impact:

    * Documentation completeness improved.
    * User/Developer guidance enhanced.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For experiment

#### Run

```bash
commiter commit -t experiment
```

#### Output

```text
EXPERIMENT: {{ commit_message }}

Details:

    * Experimental feature: {{ experimental_feature }}.
    * Hypothesis: {{ hypothesis }}.
    * Test duration: {{ test_duration }}.
    * Metrics to be tracked: {{ tracked_metrics }}.

Impact:

    * Experiment outcomes will inform: {{ outcome_impact }}.
    * Risks: {{ risks }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For feat

#### Run

```bash
commiter commit -t feat
```

#### Output

```text
FEATURE ADDITION: {{ commit_message }}

Details:

    * New feature: {{ feature_name }}.
    * Reason for addition: {{ reason_for_addition }}.
    * Affected modules: {{ affected_modules }}.

Impact:

    * User experience enhanced.
    * New capabilities available: {{ new_capabilities }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For feature-addition

#### Run

```bash
commiter commit -t feature-addition
```

#### Output

```text
FEATURE ADDITION: {{ commit_message }}

Details:

    * Feature added: {{ feature_name }}.
    * Reason for addition: {{ reason_for_addition }}.
    * Modules affected: {{ affected_modules }}.
    * Dependencies introduced: {{ dependencies_introduced }}.

Impact:

    * New functionality: {{ new_functionality }}.
    * User benefit: {{ user_benefit }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For feature-toggle

#### Run

```bash
commiter commit -t feature-toggle
```

#### Output

```text
FEATURE TOGGLE: {{ commit_message }}

Details:

    * Feature toggled: {{ feature_toggled }}.
    * Reason for toggling: {{ reason_for_toggling }}.
    * Status: {{ toggle_status }} (Enabled/Disabled).

Impact:

    * Modules affected: {{ affected_modules }}.
    * User impact: {{ user_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For fix

#### Run

```bash
commiter commit -t fix
```

#### Output

```text
FIX: {{ commit_message }}

Details:

    * Issue fixed: {{ issue_fixed }}.
    * Module affected: {{ affected_module }}.
    * Root cause: {{ root_cause }}.
    * Solution applied: {{ solution_applied }}.

Impact:

    * System stability restored.
    * Incident resolution time: {{ resolution_time }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For hotfix

#### Run

```bash
commiter commit -t hotfix
```

#### Output

```text
HOTFIX: {{ commit_message }}

Details:

    * Issue hotfixed: {{ issue_hotfixed }}.
    * Reason for hotfix: {{ reason_for_hotfix }}.
    * Affected module: {{ affected_module }}.
    * Patch applied: {{ patch_applied }}.

Impact:

    * Service disruption minimized.
    * Immediate resolution: {{ resolution_time }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For hotpatch

#### Run

```bash
commiter commit -t hotpatch
```

#### Output

```text
HOTPATCH: {{ commit_message }}

Details:

    * Hotpatch applied to: {{ system_or_service }}.
    * Issue resolved: {{ issue_resolved }}.
    * Affected module: {{ affected_module }}.

Impact:

    * No downtime required.
    * System performance restored: {{ system_performance_restored }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For log-rotation-update

#### Run

```bash
commiter commit -t log-rotation-update
```

#### Output

```text
LOG ROTATION UPDATE: {{ commit_message }}

Details:

    * Log rotation policy updated for: {{ system_or_service }}.
    * New retention period: {{ retention_period }}.
    * Log rotation frequency: {{ rotation_frequency }}.

Impact:

    * Storage space optimized.
    * Improved logging efficiency.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For log-update

#### Run

```bash
commiter commit -t log-update
```

#### Output

```text
LOG UPDATE: {{ commit_message }}

Details:

    * Log format updated for: {{ system_or_service }}.
    * New log format: {{ new_log_format }}.
    * Affected modules: {{ affected_modules }}.

Impact:

    * Enhanced log readability.
    * Improved monitoring and troubleshooting.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For merge

#### Run

```bash
commiter commit -t merge
```

#### Output

```text
MERGE: {{ commit_message }}

Details:

    * Merged branch: {{ merged_branch }} into {{ target_branch }}.
    * Conflicts resolved: {{ conflicts_resolved }}.
    * Reason for merge: {{ reason_for_merge }}.

Impact:

    * Features/bugfixes merged from: {{ source_branch }}.
    * Codebase integration improved.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For migration

#### Run

```bash
commiter commit -t migration
```

#### Output

```text
MIGRATION: {{ commit_message }}

Details:

    * Migration type: {{ migration_type }}.
    * Data migrated: {{ data_migrated }}.
    * Downtime required: {{ downtime_required }}.

Impact:

    * System functionality: {{ system_functionality_impact }}.
    * Data integrity maintained: {{ data_integrity }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For minor-update

#### Run

```bash
commiter commit -t minor-update
```

#### Output

```text
MINOR UPDATE: {{ commit_message }}

Details:

    * Minor update applied to: {{ system_or_service }}.
    * Affected modules: {{ affected_modules }}.
    * Changes: {{ changes }}.

Impact:

    * Small improvements or bugfixes.
    * No significant impact on functionality.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For monitoring-update

#### Run

```bash
commiter commit -t monitoring-update
```

#### Output

```text
MONITORING UPDATE: {{ commit_message }}

Details:

    * Monitoring system updated for: {{ system_or_service }}.
    * New monitoring thresholds: {{ new_thresholds }}.
    * Alerts configuration: {{ alerts_configuration }}.

Impact:

    * Improved system visibility.
    * Faster response to incidents.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For optimize

#### Run

```bash
commiter commit -t optimize
```

#### Output

```text
OPTIMIZATION: {{ commit_message }}

Details:

    * Module optimized: {{ optimized_module }}.
    * Performance bottleneck resolved: {{ bottleneck }}.
    * Resource usage reduced: {{ resource_usage_reduction }}.

Impact:

    * System performance improved.
    * Response time improvement: {{ response_time_improvement }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For perf

#### Run

```bash
commiter commit -t perf
```

#### Output

```text
PERFORMANCE IMPROVEMENT: {{ commit_message }}

Details:

    * Performance optimized for: {{ system_or_service }}.
    * Reason for improvement: {{ reason_for_improvement }}.
    * Bottleneck resolved: {{ bottleneck }}.

Impact:

    * Improved performance metrics.
    * System response time reduced by: {{ response_time_improvement }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For refactor

#### Run

```bash
commiter commit -t refactor
```

#### Output

```text
REFACTOR: {{ commit_message }}

Details:

    * Refactor applied to: {{ affected_module }}.
    * Reason for refactor: {{ reason_for_refactor }}.
    * Code structure improved in: {{ improved_structure }}.

Impact:

    * Code readability enhanced.
    * Performance impact: {{ performance_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For remove

#### Run

```bash
commiter commit -t remove
```

#### Output

```text
REMOVE: {{ commit_message }}

Details:

    * Feature removed: {{ removed_feature }}.
    * Reason for removal: {{ reason_for_removal }}.
    * Modules affected: {{ affected_modules }}.

Impact:

    * Unused or deprecated code eliminated.
    * Codebase simplified and optimized.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For revert

#### Run

```bash
commiter commit -t revert
```

#### Output

```text
REVERT: {{ commit_message }}

Details:

    * Reverted commit: {{ reverted_commit }}.
    * Reason for revert: {{ reason_for_revert }}.
    * Changes undone: {{ changes_undone }}.

Impact:

    * Codebase restored to a previous stable state.
    * Issues with the reverted commit resolved.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For rollback-deployment

#### Run

```bash
commiter commit -t rollback-deployment
```

#### Output

```text
ROLLBACK DEPLOYMENT: {{ commit_message }}

Details:

    * Rolled back version: {{ rolled_back_version }}.
    * Reason for rollback: {{ reason_for_rollback }}.
    * Downtime duration: {{ downtime_duration }}.

Impact:

    * Previous stable version restored.
    * System availability ensured.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For rollback

#### Run

```bash
commiter commit -t rollback
```

#### Output

```text
ROLLBACK: {{ commit_message }}

Details:

    * Rolled back changes: {{ rolled_back_changes }}.
    * Reason for rollback: {{ reason_for_rollback }}.
    * Affected modules: {{ affected_modules }}.

Impact:

    * System restored to a stable state.
    * Risk of system instability mitigated.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For schema-update

#### Run

```bash
commiter commit -t schema-update
```

#### Output

```text
SCHEMA UPDATE: {{ commit_message }}

Details:

    * Database schema updated for: {{ affected_database }}.
    * New schema version: {{ schema_version }}.
    * Tables/Columns affected: {{ affected_tables }}.

Impact:

    * Database integrity maintained.
    * System performance: {{ performance_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For secret-update

#### Run

```bash
commiter commit -t secret-update
```

#### Output

```text
SECRET UPDATE: {{ commit_message }}

Details:

    * Secret updated for: {{ system_or_service }}.
    * Secret rotation applied: {{ rotation_applied }}.
    * Expiry of previous secret: {{ expiry_date }}.

Impact:

    * Security posture improved.
    * Risk of compromised secrets mitigated.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For security-patch

#### Run

```bash
commiter commit -t security-patch
```

#### Output

```text
SECURITY PATCH: {{ commit_message }}

Details:

    * Vulnerability fixed: {{ vulnerability_fixed }}.
    * Affected modules: {{ affected_modules }}.
    * Security risk level: {{ security_risk_level }}.

Impact:

    * System security improved.
    * Compliance status: {{ compliance_status }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For security

#### Run

```bash
commiter commit -t security
```

#### Output

```text
SECURITY UPDATE: {{ commit_message }}

Details:

    * Security update applied to: {{ system_or_service }}.
    * Risk mitigated: {{ risk_mitigated }}.
    * Security protocols enhanced: {{ security_protocols }}.

Impact:

    * System protection improved.
    * Threat vectors reduced.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For services

#### Run

```bash
commiter commit -t services
```

#### Output

```text
SERVICES UPDATE: {{ commit_message }}

Details:

    * Service updated: {{ service_name }}.
    * New features introduced: {{ new_features }}.
    * Performance improvements: {{ performance_improvements }}.

Impact:

    * Service availability: {{ availability_status }}.
    * User experience enhanced.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For style

#### Run

```bash
commiter commit -t style
```

#### Output

```text
STYLE: {{ commit_message }}

Details:

    * Code style updated for: {{ affected_modules }}.
    * Linting rules applied: {{ linting_rules }}.
    * Formatting changes: {{ formatting_changes }}.

Impact:

    * Code readability improved.
    * Code consistency maintained.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For test

#### Run

```bash
commiter commit -t test
```

#### Output

```text
TEST: {{ commit_message }}

Details:

    * Test written for: {{ feature_or_module }}.
    * Test coverage improved: {{ test_coverage }}.
    * Tests passed: {{ tests_passed }}.

Impact:

    * Code reliability improved.
    * Bugs or issues detected: {{ bugs_detected }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For testing-environment-update

#### Run

```bash
commiter commit -t testing-environment-update
```

#### Output

```text
TESTING ENVIRONMENT UPDATE: {{ commit_message }}

Details:

    * Testing environment updated for: {{ system_or_service }}.
    * Test coverage improvements: {{ test_coverage }}.
    * Test suite updated: {{ test_suite_update }}.

Impact:

    * Testing efficiency improved.
    * More comprehensive test scenarios covered.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For update

#### Run

```bash
commiter commit -t update
```

#### Output

```text
UPDATE: {{ commit_message }}

Details:

    * System update applied to: {{ system_or_service }}.
    * Changes introduced: {{ changes_introduced }}.
    * Modules affected: {{ affected_modules }}.

Impact:

    * System functionality improved.
    * Performance impact: {{ performance_impact }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

### For version-bump

#### Run

```bash
commiter commit -t version-bump
```

#### Output

```text
VERSION BUMP: {{ commit_message }}

Details:

    * Version bumped from: {{ previous_version }} to {{ new_version }}.
    * Reason for version bump: {{ reason_for_bump }}.
    * Changes introduced: {{ changes_introduced }}.

Impact:

    * System compatibility ensured.
    * User notification required: {{ user_notification_required }}.

Author: {{ author }}
Date:   {{ commit_date }}
Parent Commit: {{ parent_commit }}
```

