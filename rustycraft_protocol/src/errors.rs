use log::error;
use prost::{DecodeError, EncodeError};
use std::ffi::NulError;
use std::io::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinError;
use tokio::time::error::Elapsed;
use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite)]
#[deku(type = "u32", endian = "little")]
#[repr(u32)]
pub enum WowRpcResponse {
    Ok = 0x00000000,
    Internal = 0x00000001,
    TimedOut = 0x00000002,
    Denied = 0x00000003,
    NotExists = 0x00000004,
    NotStarted = 0x00000005,
    InProgress = 0x00000006,
    InvalidArgs = 0x00000007,
    InvalidSubscriber = 0x00000008,
    WaitingForDependency = 0x00000009,
    NoAuth = 0x0000000A,
    ParentalControlRestriction = 0x0000000B,
    NoGameAccount = 0x0000000C,
    NotImplemented = 0x0000000D,
    ObjectRemoved = 0x0000000E,
    InvalidEntityId = 0x0000000F,
    InvalidEntityAccountId = 0x00000010,
    InvalidEntityGameAccountId = 0x00000011,
    InvalidAgentId = 0x00000013,
    InvalidTargetId = 0x00000014,
    ModuleNotLoaded = 0x00000015,
    ModuleNoEntryPoint = 0x00000016,
    ModuleSignatureIncorrect = 0x00000017,
    ModuleCreateFailed = 0x00000018,
    NoProgram = 0x00000019,
    ApiNotReady = 0x0000001B,
    BadVersion = 0x0000001C,
    AttributeTooManyAttributesSet = 0x0000001D,
    AttributeMaxSizeExceeded = 0x0000001E,
    AttributeQuotaExceeded = 0x0000001F,
    ServerPoolServerDisappeared = 0x00000020,
    ServerIsPrivate = 0x00000021,
    Disabled = 0x00000022,
    ModuleNotFound = 0x00000024,
    ServerBusy = 0x00000025,
    NoBattletag = 0x00000026,
    IncompleteProfanityFilters = 0x00000027,
    InvalidRegion = 0x00000028,
    ExistsAlready = 0x00000029,
    InvalidServerThumbprint = 0x0000002A,
    PhoneLock = 0x0000002B,
    Squelched = 0x0000002C,
    TargetOffline = 0x0000002D,
    BadServer = 0x0000002E,
    NoCookie = 0x0000002F,
    ExpiredCookie = 0x00000030,
    TokenNotFound = 0x00000031,
    GameAccountNoTime = 0x00000032,
    GameAccountNoPlan = 0x00000033,
    GameAccountBanned = 0x00000034,
    GameAccountSuspended = 0x00000035,
    GameAccountAlreadySelected = 0x00000036,
    GameAccountCancelled = 0x00000037,
    GameAccountCreationDisabled = 0x00000038,
    GameAccountLocked = 0x00000039,

    SessionDuplicate = 0x0000003C,
    SessionDisconnected = 0x0000003D,
    SessionDataChanged = 0x0000003E,
    SessionUpdateFailed = 0x0000003F,
    SessionNotFound = 0x00000040,

    AdminKick = 0x00000046,
    UnplannedMaintenance = 0x00000047,
    PlannedMaintenance = 0x00000048,
    ServiceFailureAccount = 0x00000049,
    ServiceFailureSession = 0x0000004A,
    ServiceFailureAuth = 0x0000004B,
    ServiceFailureRisk = 0x0000004C,
    BadProgram = 0x0000004D,
    BadLocale = 0x0000004E,
    BadPlatform = 0x0000004F,
    LocaleRestrictedLa = 0x00000051,
    LocaleRestrictedRu = 0x00000052,
    LocaleRestrictedKo = 0x00000053,
    LocaleRestrictedTw = 0x00000054,
    LocaleRestricted = 0x00000055,
    AccountNeedsMaintenance = 0x00000056,
    ModuleApiError = 0x00000057,
    ModuleBadCacheHandle = 0x00000058,
    ModuleAlreadyLoaded = 0x00000059,
    NetworkBlacklisted = 0x0000005A,
    EventProcessorSlow = 0x0000005B,
    ServerShuttingDown = 0x0000005C,
    NetworkNotPrivileged = 0x0000005D,
    TooManyOutstandingRequests = 0x0000005E,
    NoAccountRegistered = 0x0000005F,
    BattlenetAccountBanned = 0x00000060,

    OkDeprecated = 0x00000064,
    ServerInModeZombie = 0x00000065,

    LogonModuleRequired = 0x000001F4,
    LogonModuleNotConfigured = 0x000001F5,
    LogonModuleTimeout = 0x000001F6,
    LogonAgreementRequired = 0x000001FE,
    LogonAgreementNotConfigured = 0x000001FF,

    LogonInvalidServerProof = 0x00000208,
    LogonWebVerifyTimeout = 0x00000209,
    LogonInvalidAuthToken = 0x0000020A,

    ChallengeSmsTooSoon = 0x00000258,
    ChallengeSmsThrottled = 0x00000259,
    ChallengeSmsTempOutage = 0x0000025A,
    ChallengeNoChallenge = 0x0000025B,
    ChallengeNotPicked = 0x0000025C,
    ChallengeAlreadyPicked = 0x0000025D,
    ChallengeInProgress = 0x0000025E,

    ConfigFormatInvalid = 0x000002BC,
    ConfigNotFound = 0x000002BD,
    ConfigRetrieveFailed = 0x000002BE,

    NetworkModuleBusy = 0x000003E8,
    NetworkModuleCantResolveAddress = 0x000003E9,
    NetworkModuleConnectionRefused = 0x000003EA,
    NetworkModuleInterrupted = 0x000003EB,
    NetworkModuleConnectionAborted = 0x000003EC,
    NetworkModuleConnectionReset = 0x000003ED,
    NetworkModuleBadAddress = 0x000003EE,
    NetworkModuleNotReady = 0x000003EF,
    NetworkModuleAlreadyConnected = 0x000003F0,
    NetworkModuleCantCreateSocket = 0x000003F1,
    NetworkModuleNetworkUnreachable = 0x000003F2,
    NetworkModuleSocketPermissionDenied = 0x000003F3,
    NetworkModuleNotInitialized = 0x000003F4,
    NetworkModuleNoSslCertificateForPeer = 0x000003F5,
    NetworkModuleNoSslCommonNameForCertificate = 0x000003F6,
    NetworkModuleSslCommonNameDoesNotMatchRemoteEndpoint = 0x000003F7,
    NetworkModuleSocketClosed = 0x000003F8,
    NetworkModuleSslPeerIsNotRegisteredInCertbundle = 0x000003F9,
    NetworkModuleSslInitializeLowFirst = 0x000003FA,
    NetworkModuleSslCertBundleReadError = 0x000003FB,
    NetworkModuleNoCertBundle = 0x000003FC,
    NetworkModuleFailedToDownloadCertBundle = 0x000003FD,
    NetworkModuleNotReadyToRead = 0x000003FE,

    NetworkModuleOpensslX509Ok = 0x000004B0,
    NetworkModuleOpensslX509UnableToGetIssuerCert = 0x000004B1,
    NetworkModuleOpensslX509UnableToGetCrl = 0x000004B2,
    NetworkModuleOpensslX509UnableToDecryptCertSignature = 0x000004B3,
    NetworkModuleOpensslX509UnableToDecryptCrlSignature = 0x000004B4,
    NetworkModuleOpensslX509UnableToDecodeIssuerPublicKey = 0x000004B5,
    NetworkModuleOpensslX509CertSignatureFailure = 0x000004B6,
    NetworkModuleOpensslX509CrlSignatureFailure = 0x000004B7,
    NetworkModuleOpensslX509CertNotYetValid = 0x000004B8,
    NetworkModuleOpensslX509CertHasExpired = 0x000004B9,
    NetworkModuleOpensslX509CrlNotYetValid = 0x000004BA,
    NetworkModuleOpensslX509CrlHasExpired = 0x000004BB,
    NetworkModuleOpensslX509InCertNotBeforeField = 0x000004BC,
    NetworkModuleOpensslX509InCertNotAfterField = 0x000004BD,
    NetworkModuleOpensslX509InCrlLastUpdateField = 0x000004BE,
    NetworkModuleOpensslX509InCrlNextUpdateField = 0x000004BF,
    NetworkModuleOpensslX509OutOfMem = 0x000004C0,
    NetworkModuleOpensslX509DepthZeroSelfSignedCert = 0x000004C1,
    NetworkModuleOpensslX509SelfSignedCertInChain = 0x000004C2,
    NetworkModuleOpensslX509UnableToGetIssuerCertLocally = 0x000004C3,
    NetworkModuleOpensslX509UnableToVerifyLeafSignature = 0x000004C4,
    NetworkModuleOpensslX509CertChainTooLong = 0x000004C5,
    NetworkModuleOpensslX509CertRevoked = 0x000004C6,
    NetworkModuleOpensslX509InvalidCa = 0x000004C7,
    NetworkModuleOpensslX509PathLengthExceeded = 0x000004C8,
    NetworkModuleOpensslX509InvalidPurpose = 0x000004C9,
    NetworkModuleOpensslX509CertUntrusted = 0x000004CA,
    NetworkModuleOpensslX509CertRejected = 0x000004CB,
    NetworkModuleOpensslX509SubjectIssuerMismatch = 0x000004CC,
    NetworkModuleOpensslX509AkidSkidMismatch = 0x000004CD,
    NetworkModuleOpensslX509AkidIssuerSerialMismatch = 0x000004CE,
    NetworkModuleOpensslX509KeyusageNoCertsign = 0x000004CF,
    NetworkModuleOpensslX509ApplicationVerification = 0x000004D0,

    NetworkModuleSchannelCannotFindOsVersion = 0x00000514,
    NetworkModuleSchannelOsNotSupported = 0x00000515,
    NetworkModuleSchannelLoadlibraryFail = 0x00000516,
    NetworkModuleSchannelCannotFindInterface = 0x00000517,
    NetworkModuleSchannelInitFail = 0x00000518,
    NetworkModuleSchannelFunctionCallFail = 0x00000519,
    NetworkModuleSchannelX509UnableToGetIssuerCert = 0x00000546,
    NetworkModuleSchannelX509TimeInvalid = 0x00000547,
    NetworkModuleSchannelX509SignatureInvalid = 0x00000548,
    NetworkModuleSchannelX509UnableToVerifyLeafSignature = 0x00000549,
    NetworkModuleSchannelX509SelfSignedLeafCertificate = 0x0000054A,
    NetworkModuleSchannelX509UnhandledError = 0x0000054B,
    NetworkModuleSchannelX509SelfSignedCertInChain = 0x0000054C,

    WebsocketHandshake = 0x00000578,

    NetworkModuleDurangoUnknown = 0x000005DC,
    NetworkModuleDurangoMalformedHostName = 0x000005DD,
    NetworkModuleDurangoInvalidConnectionResponse = 0x000005DE,
    NetworkModuleDurangoInvalidCaCert = 0x000005DF,

    RpcWriteFailed = 0x00000BB8,
    RpcServiceNotBound = 0x00000BB9,
    RpcTooManyRequests = 0x00000BBA,
    RpcPeerUnknown = 0x00000BBB,
    RpcPeerUnavailable = 0x00000BBC,
    RpcPeerDisconnected = 0x00000BBD,
    RpcRequestTimedOut = 0x00000BBE,
    RpcConnectionTimedOut = 0x00000BBF,
    RpcMalformedResponse = 0x00000BC0,
    RpcAccessDenied = 0x00000BC1,
    RpcInvalidService = 0x00000BC2,
    RpcInvalidMethod = 0x00000BC3,
    RpcInvalidObject = 0x00000BC4,
    RpcMalformedRequest = 0x00000BC5,
    RpcQuotaExceeded = 0x00000BC6,
    RpcNotImplemented = 0x00000BC7,
    RpcServerError = 0x00000BC8,
    RpcShutdown = 0x00000BC9,
    RpcDisconnect = 0x00000BCA,
    RpcDisconnectIdle = 0x00000BCB,
    RpcProtocolError = 0x00000BCC,
    RpcNotReady = 0x00000BCD,
    RpcForwardFailed = 0x00000BCE,
    RpcEncryptionFailed = 0x00000BCF,
    RpcInvalidAddress = 0x00000BD0,
    RpcMethodDisabled = 0x00000BD1,
    RpcShardNotFound = 0x00000BD2,
    RpcInvalidConnectionId = 0x00000BD3,
    RpcNotConnected = 0x00000BD4,
    RpcInvalidConnectionState = 0x00000BD5,
    RpcServiceAlreadyRegistered = 0x00000BD6,

    PresenceInvalidFieldId = 0x00000FA0,
    PresenceNoValidSubscribers = 0x00000FA1,
    PresenceAlreadySubscribed = 0x00000FA2,
    PresenceConsumerNotFound = 0x00000FA3,
    PresenceConsumerIsNull = 0x00000FA4,
    PresenceTemporaryOutage = 0x00000FA5,
    PresenceTooManySubscriptions = 0x00000FA6,
    PresenceSubscriptionCancelled = 0x00000FA7,
    PresenceRichPresenceParseError = 0x00000FA8,
    PresenceRichPresenceXmlError = 0x00000FA9,
    PresenceRichPresenceLoadError = 0x00000FAA,

    FriendsTooManySentInvitations = 0x00001389,
    FriendsTooManyReceivedInvitations = 0x0000138A,
    FriendsFriendshipAlreadyExists = 0x0000138B,
    FriendsFriendshipDoesNotExist = 0x0000138C,
    FriendsInvitationAlreadyExists = 0x0000138D,
    FriendsInvalidInvitation = 0x0000138E,
    FriendsAlreadySubscribed = 0x0000138F,
    FriendsAccountBlocked = 0x00001391,
    FriendsNotSubscribed = 0x00001392,
    FriendsInvalidRoleId = 0x00001393,
    FriendsDisabledRoleId = 0x00001394,
    FriendsNoteMaxSizeExceeded = 0x00001395,
    FriendsUpdateFriendStateFailed = 0x00001396,
    FriendsInviteeAtMaxFriends = 0x00001397,
    FriendsInviterAtMaxFriends = 0x00001398,

    PlatformStorageFileWriteDenied = 0x00001770,

    WhisperUndeliverable = 0x00001B58,
    WhisperMaxSizeExceeded = 0x00001B59,

    UserManagerAlreadyBlocked = 0x00001F40,
    UserManagerNotBlocked = 0x00001F41,
    UserManagerCannotBlockSelf = 0x00001F42,
    UserManagerAlreadyRegistered = 0x00001F43,
    UserManagerNotRegistered = 0x00001F44,
    UserManagerTooManyBlockedEntities = 0x00001F45,
    UserManagerTooManyIds = 0x00001F47,
    UserManagerBlockRecordUnavailable = 0x00001F4F,
    UserManagerBlockEntityFailed = 0x00001F50,
    UserManagerUnblockEntityFailed = 0x00001F51,
    UserManagerCannotBlockFriend = 0x00001F53,

    SocialNetworkDbException = 0x00002328,
    SocialNetworkDenialFromProvider = 0x00002329,
    SocialNetworkInvalidSnsId = 0x0000232A,
    SocialNetworkCantSendToProvider = 0x0000232B,
    SocialNetworkExCommFailed = 0x0000232C,
    SocialNetworkDisabled = 0x0000232D,
    SocialNetworkMissingRequestParam = 0x0000232E,
    SocialNetworkUnsupportedOauthVersion = 0x0000232F,

    ChannelFull = 0x00002710,
    ChannelNoChannel = 0x00002711,
    ChannelNotMember = 0x00002712,
    ChannelAlreadyMember = 0x00002713,
    ChannelNoSuchMember = 0x00002714,
    ChannelInvalidChannelId = 0x00002716,
    ChannelNoSuchInvitation = 0x00002718,
    ChannelTooManyInvitations = 0x00002719,
    ChannelInvitationAlreadyExists = 0x0000271A,
    ChannelInvalidChannelSize = 0x0000271B,
    ChannelInvalidRoleId = 0x0000271C,
    ChannelRoleNotAssignable = 0x0000271D,
    ChannelInsufficientPrivileges = 0x0000271E,
    ChannelInsufficientPrivacyLevel = 0x0000271F,
    ChannelInvalidPrivacyLevel = 0x00002720,
    ChannelTooManyChannelsJoined = 0x00002721,
    ChannelInvitationAlreadySubscribed = 0x00002722,
    ChannelInvalidChannelDelegate = 0x00002723,
    ChannelSlotAlreadyReserved = 0x00002724,
    ChannelSlotNotReserved = 0x00002725,
    ChannelNoReservedSlotsAvailable = 0x00002726,
    ChannelInvalidRoleSet = 0x00002727,
    ChannelRequireFriendValidation = 0x00002728,
    ChannelMemberOffline = 0x00002729,
    ChannelReceivedTooManyInvitations = 0x0000272A,
    ChannelInvitationInvalidGameAccountSelected = 0x0000272B,
    ChannelUnreachable = 0x0000272C,
    ChannelInvitationNotSubscribed = 0x0000272D,
    ChannelInvalidMessageSize = 0x0000272E,
    ChannelMaxMessageSizeExceeded = 0x0000272F,
    ChannelConfigNotFound = 0x00002730,
    ChannelInvalidChannelType = 0x00002731,

    LocalStorageFileOpenError = 0x00002AF8,
    LocalStorageFileCreateError = 0x00002AF9,
    LocalStorageFileReadError = 0x00002AFA,
    LocalStorageFileWriteError = 0x00002AFB,
    LocalStorageFileDeleteError = 0x00002AFC,
    LocalStorageFileCopyError = 0x00002AFD,
    LocalStorageFileDecompressError = 0x00002AFE,
    LocalStorageFileHashMismatch = 0x00002AFF,
    LocalStorageFileUsageMismatch = 0x00002B00,
    LocalStorageDatabaseInitError = 0x00002B01,
    LocalStorageDatabaseNeedsRebuild = 0x00002B02,
    LocalStorageDatabaseInsertError = 0x00002B03,
    LocalStorageDatabaseLookupError = 0x00002B04,
    LocalStorageDatabaseUpdateError = 0x00002B05,
    LocalStorageDatabaseDeleteError = 0x00002B06,
    LocalStorageDatabaseShrinkError = 0x00002B07,
    LocalStorageCacheCrawlError = 0x00002B08,
    LocalStorageDatabaseIndexTriggerError = 0x00002B09,
    LocalStorageDatabaseRebuildInProgress = 0x00002B0A,
    LocalStorageOkButNotInCache = 0x00002B0B,
    LocalStorageDatabaseRebuildInterrupted = 0x00002B0D,
    LocalStorageDatabaseNotInitialized = 0x00002B0E,
    LocalStorageDirectoryCreateError = 0x00002B0F,
    LocalStorageFilekeyNotFound = 0x00002B10,
    LocalStorageNotAvailableOnServer = 0x00002B11,

    RegistryCreateKeyError = 0x00002EE0,
    RegistryOpenKeyError = 0x00002EE1,
    RegistryReadError = 0x00002EE2,
    RegistryWriteError = 0x00002EE3,
    RegistryTypeError = 0x00002EE4,
    RegistryDeleteError = 0x00002EE5,
    RegistryEncryptError = 0x00002EE6,
    RegistryDecryptError = 0x00002EE7,
    RegistryKeySizeError = 0x00002EE8,
    RegistryValueSizeError = 0x00002EE9,
    RegistryNotFound = 0x00002EEB,
    RegistryMalformedString = 0x00002EEC,

    InterfaceAlreadyConnected = 0x000032C8,
    InterfaceNotReady = 0x000032C9,
    InterfaceOptionKeyTooLarge = 0x000032CA,
    InterfaceOptionValueTooLarge = 0x000032CB,
    InterfaceOptionKeyInvalidUtf8String = 0x000032CC,
    InterfaceOptionValueInvalidUtf8String = 0x000032CD,

    HttpCouldntResolve = 0x000036B0,
    HttpCouldntConnect = 0x000036B1,
    HttpTimeout = 0x000036B2,
    HttpFailed = 0x000036B3,
    HttpMalformedUrl = 0x000036B4,
    HttpDownloadAborted = 0x000036B5,
    HttpCouldntWriteFile = 0x000036B6,
    HttpTooManyRedirects = 0x000036B7,
    HttpCouldntOpenFile = 0x000036B8,
    HttpCouldntCreateFile = 0x000036B9,
    HttpCouldntReadFile = 0x000036BA,
    HttpCouldntRenameFile = 0x000036BB,
    HttpCouldntCreateDirectory = 0x000036BC,
    HttpCurlIsNotReady = 0x000036BD,
    HttpCancelled = 0x000036BE,

    HttpFileNotFound = 0x00003844,

    AccountMissingConfig = 0x00004650,
    AccountDataNotFound = 0x00004651,
    AccountAlreadySubscribed = 0x00004652,
    AccountNotSubscribed = 0x00004653,
    AccountFailedToParseTimezoneData = 0x00004654,
    AccountLoadFailed = 0x00004655,
    AccountLoadCancelled = 0x00004656,
    AccountDatabaseInvalidateFailed = 0x00004657,
    AccountCacheInvalidateFailed = 0x00004658,
    AccountSubscriptionPending = 0x00004659,
    AccountUnknownRegion = 0x0000465A,
    AccountDataFailedToParse = 0x0000465B,
    AccountUnderage = 0x0000465C,
    AccountIdentityCheckPending = 0x0000465D,
    AccountIdentityUnverified = 0x0000465E,

    DatabaseBindingCountMismatch = 0x00004A38,
    DatabaseBindingParseFail = 0x00004A39,
    DatabaseResultsetColumnsMismatch = 0x00004A3A,
    DatabaseDeadlock = 0x00004A3B,
    DatabaseDuplicateKey = 0x00004A3C,
    DatabaseCannotConnect = 0x00004A3D,
    DatabaseStatementFailed = 0x00004A3E,
    DatabaseTransactionNotStarted = 0x00004A3F,
    DatabaseTransactionNotEnded = 0x00004A40,
    DatabaseTransactionLeak = 0x00004A41,
    DatabaseTransactionStateBad = 0x00004A42,
    DatabaseServerGone = 0x00004A43,
    DatabaseQueryTimeout = 0x00004A44,
    DatabaseBindingNotNullable = 0x00004A9C,
    DatabaseBindingInvalidInteger = 0x00004A9D,
    DatabaseBindingInvalidFloat = 0x00004A9E,
    DatabaseBindingInvalidTemporal = 0x00004A9F,
    DatabaseBindingInvalidProtobuf = 0x00004AA0,

    PartyInvalidPartyId = 0x00004E20,
    PartyAlreadyInParty = 0x00004E21,
    PartyNotInParty = 0x00004E22,
    PartyInvitationUndeliverable = 0x00004E23,
    PartyInvitationAlreadyExists = 0x00004E24,
    PartyTooManyPartyInvitations = 0x00004E25,
    PartyTooManyReceivedInvitations = 0x00004E26,
    PartyNoSuchType = 0x00004E27,

    GamesNoSuchFactory = 0x000055F0,
    GamesNoSuchGame = 0x000055F1,
    GamesNoSuchRequest = 0x000055F2,
    GamesNoSuchPartyMember = 0x000055F3,

    ResourcesOffline = 0x000059D8,

    GameServerCreateGameRefused = 0x00005DC0,
    GameServerAddPlayersRefused = 0x00005DC1,
    GameServerRemovePlayersRefused = 0x00005DC2,
    GameServerFinishGameRefused = 0x00005DC3,
    GameServerNoSuchGame = 0x00005DC4,
    GameServerNoSuchPlayer = 0x00005DC5,
    GameServerCreateGameRefusedTransient = 0x00005DF2,
    GameServerAddPlayersRefusedTransient = 0x00005DF3,
    GameServerRemovePlayersRefusedTransient = 0x00005DF4,
    GameServerFinishGameRefusedTransient = 0x00005DF5,
    GameServerCreateGameRefusedBusy = 0x00005E24,
    GameServerAddPlayersRefusedBusy = 0x00005E25,
    GameServerRemovePlayersRefusedBusy = 0x00005E26,
    GameServerFinishGameRefusedBusy = 0x00005E27,

    GameMasterInvalidFactory = 0x000061A8,
    GameMasterInvalidGame = 0x000061A9,
    GameMasterGameFull = 0x000061AA,
    GameMasterRegisterFailed = 0x000061AB,
    GameMasterNoGameServer = 0x000061AC,
    GameMasterNoUtilityServer = 0x000061AD,
    GameMasterNoGameVersion = 0x000061AE,
    GameMasterGameJoinFailed = 0x000061AF,
    GameMasterAlreadyRegistered = 0x000061B0,
    GameMasterNoFactory = 0x000061B1,
    GameMasterMultipleGameVersions = 0x000061B2,
    GameMasterInvalidPlayer = 0x000061B3,
    GameMasterInvalidGameRequest = 0x000061B4,
    GameMasterInsufficientPrivileges = 0x000061B5,
    GameMasterAlreadyInGame = 0x000061B6,
    GameMasterInvalidGameServerResponse = 0x000061B7,
    GameMasterGameAccountLookupFailed = 0x000061B8,
    GameMasterGameEntryCancelled = 0x000061B9,
    GameMasterGameEntryAbortedClientDropped = 0x000061BA,
    GameMasterGameEntryAbortedByService = 0x000061BB,
    GameMasterNoAvailableCapacity = 0x000061BC,
    GameMasterInvalidTeamId = 0x000061BD,
    GameMasterCreationInProgress = 0x000061BE,

    NotificationInvalidClientId = 0x00006590,
    NotificationDuplicateName = 0x00006591,
    NotificationNameNotFound = 0x00006592,
    NotificationInvalidServer = 0x00006593,
    NotificationQuotaExceeded = 0x00006594,
    NotificationInvalidNotificationType = 0x00006595,
    NotificationUndeliverable = 0x00006596,
    NotificationUndeliverableTemporary = 0x00006597,

    AchievementsNothingToUpdate = 0x00006D60,
    AchievementsInvalidParams = 0x00006D61,
    AchievementsNotRegistered = 0x00006D62,
    AchievementsNotReady = 0x00006D63,
    AchievementsFailedToParseStaticData = 0x00006D64,
    AchievementsUnknownId = 0x00006D65,
    AchievementsMissingSnapshot = 0x00006D66,
    AchievementsAlreadyRegistered = 0x00006D67,
    AchievementsTooManyRegistrations = 0x00006D68,
    AchievementsAlreadyInProgress = 0x00006D69,
    AchievementsTemporaryOutage = 0x00006D6A,
    AchievementsInvalidProgramid = 0x00006D6B,
    AchievementsMissingRecord = 0x00006D6C,
    AchievementsRegistrationPending = 0x00006D6D,
    AchievementsEntityIdNotFound = 0x00006D6E,
    AchievementsAchievementIdNotFound = 0x00006D6F,
    AchievementsCriteriaIdNotFound = 0x00006D70,
    AchievementsStaticDataMismatch = 0x00006D71,
    AchievementsWrongThread = 0x00006D72,
    AchievementsCallbackIsNull = 0x00006D73,
    AchievementsAutoRegisterPending = 0x00006D74,
    AchievementsNotInitialized = 0x00006D75,
    AchievementsAchievementIdAlreadyExists = 0x00006D76,
    AchievementsFailedToDownloadStaticData = 0x00006D77,
    AchievementsStaticDataNotFound = 0x00006D78,

    GameUtilityServerVariableRequestRefused = 0x000084D1,
    GameUtilityServerWrongNumberOfVariablesReturned = 0x000084D2,
    GameUtilityServerClientRequestRefused = 0x000084D3,
    GameUtilityServerPresenceChannelCreatedRefused = 0x000084D4,
    GameUtilityServerVariableRequestRefusedTransient = 0x00008502,
    GameUtilityServerClientRequestRefusedTransient = 0x00008503,
    GameUtilityServerPresenceChannelCreatedRefusedTransient = 0x00008504,
    GameUtilityServerServerRequestRefusedTransient = 0x00008505,
    GameUtilityServerVariableRequestRefusedBusy = 0x00008534,
    GameUtilityServerClientRequestRefusedBusy = 0x00008535,
    GameUtilityServerPresenceChannelCreatedRefusedBusy = 0x00008536,
    GameUtilityServerServerRequestRefusedBusy = 0x00008537,
    GameUtilityServerNoServer = 0x00008598,

    IdentityInsufficientData = 0x0000A028,
    IdentityTooManyResults = 0x0000A029,
    IdentityBadId = 0x0000A02A,
    IdentityNoAccountBlob = 0x0000A02B,

    RiskChallengeAction = 0x0000A410,
    RiskDelayAction = 0x0000A411,
    RiskThrottleAction = 0x0000A412,
    RiskAccountLocked = 0x0000A413,
    RiskCsDenied = 0x0000A414,
    RiskDisconnectAccount = 0x0000A415,
    RiskCheckSkipped = 0x0000A416,

    ReportUnavailable = 0x0000AFC8,
    ReportTooLarge = 0x0000AFC9,
    ReportUnknownType = 0x0000AFCA,
    ReportAttributeInvalid = 0x0000AFCB,
    ReportAttributeQuotaExceeded = 0x0000AFCC,
    ReportUnconfirmed = 0x0000AFCD,
    ReportNotConnected = 0x0000AFCE,
    ReportRejected = 0x0000AFCF,
    ReportTooManyRequests = 0x0000AFD0,

    AccountAlreadyRegisterd = 0x0000BB80,
    AccountNotRegistered = 0x0000BB81,
    AccountRegistrationPending = 0x0000BB82,

    MemcachedClientNoError = 0x00010000,
    MemcachedClientKeyNotFound = 0x00010001,
    MemcachedKeyExists = 0x00010002,
    MemcachedValueToLarge = 0x00010003,
    MemcachedInvalidArgs = 0x00010004,
    MemcachedItemNotStored = 0x00010005,
    MemcachedNonNumericValue = 0x00010006,
    MemcachedWrongServer = 0x00010007,
    MemcachedAuthenticationError = 0x00010008,
    MemcachedAuthenticationContinue = 0x00010009,
    MemcachedUnknownCommand = 0x0001000A,
    MemcachedOutOfMemory = 0x0001000B,
    MemcachedNotSupported = 0x0001000C,
    MemcachedInternalError = 0x0001000D,
    MemcachedTemporaryFailure = 0x0001000E,

    MemcachedClientAlreadyConnected = 0x000186A0,
    MemcachedClientBadConfig = 0x000186A1,
    MemcachedClientNotConnected = 0x000186A2,
    MemcachedClientTimeout = 0x000186A3,
    MemcachedClientAborted = 0x000186A4,

    UtilServerFailedToSerialize = 0x80000064,
    UtilServerDisconnectedFromBattlenet = 0x80000065,
    UtilServerTimedOut = 0x80000066,
    UtilServerNoMeteringData = 0x80000067,
    UtilServerFailPermissionCheck = 0x80000068,
    UtilServerUnknownRealm = 0x80000069,
    UtilServerMissingSessionKey = 0x8000006A,
    UtilServerMissingVirtualRealm = 0x8000006B,
    UtilServerInvalidSessionKey = 0x8000006C,
    UtilServerMissingRealmList = 0x8000006D,
    UtilServerInvalidIdentityArgs = 0x8000006E,
    UtilServerSessionObjectMissing = 0x8000006F,
    UtilServerInvalidBnetSession = 0x80000070,
    UtilServerInvalidVirtualRealm = 0x80000071,
    UtilServerInvalidClientAddress = 0x80000072,
    UtilServerFailedToSerializeResponse = 0x80000073,
    UtilServerUnknownRequest = 0x80000074,
    UtilServerUnableToGenerateJoinTicket = 0x80000075,
    UtilServerUnableToGenerateRealmListTicket = 0x80000076,
    UtilServerAccountDenied = 0x80000077,
    UtilServerInvalidWowAccount = 0x80000078,
    UtilServerUnableToStoreSession = 0x80000079,
    UtilServerSessionAlreadyCreated = 0x8000007A,

    UserServerFailedToSerialize = 0x800000C8,
    UserServerDisconnectedFromUtil = 0x800000C9,
    UserServerSessionDuplicate = 0x800000CA,
    UserServerFailedToDisableBilling = 0x800000CB,
    UserServerPlayerDisconnected = 0x800000CC,
    UserServerFailedToParseAccountState = 0x800000CD,
    UserServerAccountLoadCancelled = 0x800000CE,
    UserServerBadPlatform = 0x800000CF,
    UserServerBadVirtualRealm = 0x800000D0,
    UserServerLocaleRestricted = 0x800000D1,
    UserServerMissingPropass = 0x800000D2,
    UserServerBadWowAccount = 0x800000D3,
    UserServerBadBnetAccount = 0x800000D4,
    UserServerFailedToParseGameAccountState = 0x800000D5,
    UserServerFailedToParseGameTimeRemaining = 0x800000D6,
    UserServerFailedToParseGameSessionInfo = 0x800000D7,
    UserServerAccountStatePoorlyFormed = 0x800000D8,
    UserServerGameAccountStatePoorlyFormed = 0x800000D9,
    UserServerGameTimeRemainingPoorlyFormed = 0x800000DA,
    UserServerGameSessionInfoPoorlyFormed = 0x800000DB,
    UserServerBadSessionTrackerState = 0x800000DC,
    UserServerFailedToParseCaisInfo = 0x800000DD,
    UserServerGameSessionDisconnected = 0x800000DE,
    UserServerVersionMismatch = 0x800000DF,
    UserServerAccountSuspended = 0x800000E0,
    UserServerNotPermittedOnRealm = 0x800000E1,
    UserServerLoginFailedConnect = 0x800000E2,

    WowServicesTimedOut = 0x8000012C,
    WowServicesInvalidRealmListTicket = 0x8000012D,
    WowServicesInvalidJoinTicket = 0x8000012E,
    WowServicesInvalidServerAddresses = 0x8000012F,
    WowServicesInvalidSecretBlob = 0x80000130,
    WowServicesNoRealmJoinIpFound = 0x80000131,
    WowServicesDeniedRealmListTicket = 0x80000132,
    WowServicesMissingGameAccount = 0x80000133,
    WowServicesLogonInvalidAuthToken = 0x80000134,
    WowServicesNoAvailableRealms = 0x80000135,
    WowServicesFailedToParseDispatch = 0x80000136,
    WowServicesMissingMeteringFile = 0x80000137,
    WowServicesLoginInvalidContentType = 0x80000138,
    WowServicesLoginUnableToDecode = 0x80000139,
    WowServicesLoginPostError = 0x8000013A,
    WowServicesAuthenticatorParseFailed = 0x8000013B,
    WowServicesLegalParseFailed = 0x8000013C,
    WowServicesLoginAuthenticationParseFailed = 0x8000013D,
    WowSerivcesUserMustAcceptLegal = 0x8000013E,
    WowServicesDisconnected = 0x8000013F,
    WowServicesNoHandlerForDispatch = 0x80000140,
    WowServicesPreDispatchHandlerFailed = 0x80000141,
    WowServicesCriticalStreamingError = 0x80000142,
    WowServicesWorldLoadError = 0x80000143,
    WowServicesLoginFailed = 0x80000144,
    WowServicesLoginFailedOnChallenge = 0x80000145,
    WowServicesNoPrepaidTime = 0x80000146,
    WowServicesSubscriptionExpired = 0x80000147,
    WowServicesCantConnect = 0x80000148,
}

impl From<prost::EncodeError> for WowRpcResponse {
    fn from(_: EncodeError) -> Self {
        Self::RpcMalformedResponse
    }
}

impl From<std::io::Error> for WowRpcResponse {
    fn from(_: Error) -> Self {
        Self::RpcMalformedRequest
    }
}

impl From<DecodeError> for WowRpcResponse {
    fn from(e: DecodeError) -> Self {
        error!("{}", e.to_string());
        Self::RpcMalformedRequest
    }
}

impl From<Elapsed> for WowRpcResponse {
    fn from(_: Elapsed) -> Self {
        Self::TimedOut
    }
}

impl From<NulError> for WowRpcResponse {
    fn from(_: NulError) -> Self {
        Self::Internal
    }
}

impl<T> From<SendError<T>> for WowRpcResponse {
    fn from(_: SendError<T>) -> Self {
        Self::Internal
    }
}

impl From<JoinError> for WowRpcResponse {
    fn from(_: JoinError) -> Self {
        Self::Internal
    }
}
