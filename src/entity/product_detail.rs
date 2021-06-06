use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Welcome10 {
    #[serde(rename = "document")]
    document: Document,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Document {
    // #[serde(rename = "id")]
    // pub id: SetIdClass,

    // #[serde(rename = "code")]
    // code: FormCodeClass,

    // #[serde(rename = "title")]
    // title: DocumentTitle,
    //
    // #[serde(rename = "effectiveTime")]
    // effective_time: EffectiveTime,

    #[serde(rename = "setId")]
    pub set_id: SetId,
    //
    // #[serde(rename = "versionNumber")]
    // version_number: EffectiveTime,
    //
    // #[serde(rename = "author")]
    // author: DocumentAuthor,
    //
    #[serde(rename = "component")]
    pub component: Option<DocumentComponent>,
    //
    // #[serde(rename = "xmlns")]
    // xmlns: String,
    //
    // #[serde(rename = "xmlns:xsi")]
    // xmlns_xsi: String,
    //
    // #[serde(rename = "xsi:schemaLocation")]
    // xsi_schema_location: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct DocumentAuthor {
    #[serde(rename = "assignedEntity")]
    assigned_entity: AuthorAssignedEntity,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AuthorAssignedEntity {
    #[serde(rename = "representedOrganization")]
    represented_organization: RepresentedOrganization,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct RepresentedOrganization {
    #[serde(rename = "id")]
    id: AssignedOrganizationId,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "assignedEntity")]
    assigned_entity: RepresentedOrganizationAssignedEntity,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct RepresentedOrganizationAssignedEntity {
    #[serde(rename = "assignedOrganization")]
    assigned_organization: PurpleAssignedOrganization,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleAssignedOrganization {
    #[serde(rename = "assignedEntity")]
    assigned_entity: AssignedOrganizationAssignedEntity,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AssignedOrganizationAssignedEntity {
    #[serde(rename = "assignedOrganization")]
    assigned_organization: FluffyAssignedOrganization,

    #[serde(rename = "performance")]
    performance: Vec<Performance>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyAssignedOrganization {
    #[serde(rename = "id")]
    id: AssignedOrganizationId,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AssignedOrganizationId {
    #[serde(rename = "extension")]
    extension: String,

    #[serde(rename = "root")]
    root: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Performance {
    #[serde(rename = "actDefinition")]
    act_definition: ActDefinition,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ActDefinition {
    #[serde(rename = "code")]
    code: Option<FormCode>,

    #[serde(rename = "product")]
    product: Option<Product>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FormCode {
    // #[serde(rename = "code")]
    // code: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    // #[serde(rename = "codeSystem")]
    // code_system: FormCodeCodeSystem,

    // #[serde(rename = "xsi:type")]
    // xsi_type: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "manufacturedProduct")]
    manufactured_product: ProductManufacturedProduct,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ProductManufacturedProduct {
    #[serde(rename = "manufacturedMaterialKind")]
    manufactured_material_kind: DefiningMaterialKind,

    #[serde(rename = "classCode")]
    class_code: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct DefiningMaterialKind {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct DefiningMaterialKindCode {
    #[serde(rename = "code")]
    pub code: String,

    // #[serde(rename = "codeSystem")]
    // code_system: PurpleCodeSystem,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct DocumentComponent {
    #[serde(rename = "structuredBody")]
    pub structured_body: Option<StructuredBody>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StructuredBody {
    #[serde(rename = "component")]
    pub component: Option<Vec<StructuredBodyComponent>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StructuredBodyComponent {
    #[serde(rename = "section")]
    pub section: Option<Section>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Section {
    // #[serde(rename = "id")]
    // section_id: SetIdClass,

    // #[serde(rename = "code")]
    // code: Option<FormCodeClass>,

    // #[serde(rename = "effectiveTime")]
    // effective_time: EffectiveTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subject")]
    pub subject: Option<Vec<Subject>>,

    // #[serde(rename = "excerpt")]
    // excerpt: Option<Excerpt>,

    // #[serde(rename = "ID")]
    // id: Option<String>,

    // #[serde(rename = "title")]
    // title: Option<String>,

    // #[serde(rename = "component")]
    // component: Option<ComponentUnion>,

    // #[serde(rename = "text")]
    // text: Option<TentacledText>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleComponent {
    #[serde(rename = "section")]
    section: FluffySection,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffySection {
    #[serde(rename = "id")]
    section_id: SetId,

    #[serde(rename = "code")]
    code: FormCode,

    #[serde(rename = "title")]
    title: TitleUnion,

    #[serde(rename = "text")]
    text: PurpleText,

    #[serde(rename = "effectiveTime")]
    effective_time: EffectiveTime,

    #[serde(rename = "ID")]
    id: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct EffectiveTime {
    #[serde(rename = "value")]
    value: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct SetId {
    #[serde(rename = "root")]
    pub root: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleText {
    #[serde(rename = "paragraph")]
    paragraph: Paragraph3,

    #[serde(rename = "table")]
    table: Option<TableUnion>,

    #[serde(rename = "list")]
    list: Option<MischievousList>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleList {
    #[serde(rename = "item")]
    item: Vec<PurpleItem>,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "list")]
    list: Option<FluffyList>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyList {
    #[serde(rename = "item")]
    item: Vec<FluffyItem>,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledList {
    #[serde(rename = "item")]
    item: Vec<TentacledItem>,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledItem {
    #[serde(rename = "caption")]
    caption: String,

    #[serde(rename = "list")]
    list: Option<StickyList>,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "sup")]
    sup: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyList {
    #[serde(rename = "item")]
    item: Vec<StickyItem>,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "content")]
    content: Option<TitleContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TitleContent {
    #[serde(rename = "styleCode")]
    style_code: ContentStyleCode,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleParagraph {
    #[serde(rename = "content")]
    content: Option<StickyContent>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "sup")]
    sup: Option<Br>,

    #[serde(rename = "sub")]
    sub: Option<Br>,

    #[serde(rename = "br")]
    br: Option<Vec<String>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleContent {
    #[serde(rename = "styleCode")]
    style_code: ContentStyleCode,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "linkHtml")]
    link_html: Option<LinkHtmlUnion>,

    #[serde(rename = "content")]
    content: Option<TitleContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct LinkHtmlElement {
    #[serde(rename = "href")]
    href: String,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyParagraph {
    #[serde(rename = "content")]
    content: Option<IndigoContent>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "sup")]
    sup: Option<Vec<String>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyContent {
    #[serde(rename = "linkHtml")]
    link_html: Option<LinkHtmlElement>,

    #[serde(rename = "styleCode")]
    style_code: ContentStyleCode,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleTable {
    #[serde(rename = "caption")]
    caption: PurpleCaption,

    #[serde(rename = "col")]
    col: Vec<Col>,

    #[serde(rename = "tbody")]
    tbody: PurpleTbody,

    #[serde(rename = "cellpadding")]
    cellpadding: Celling,

    #[serde(rename = "cellspacing")]
    cellspacing: Celling,

    #[serde(rename = "width")]
    width: Width,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct CaptionClass {
    #[serde(rename = "sup")]
    sup: String,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Col {
    #[serde(rename = "width")]
    width: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleTbody {
    #[serde(rename = "tr")]
    tr: Vec<PurpleTr>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleTr {
    #[serde(rename = "td")]
    td: CunningTd,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleTd {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph5>,

    #[serde(rename = "align")]
    align: Option<Align>,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "rowspan")]
    rowspan: Option<String>,

    #[serde(rename = "colspan")]
    colspan: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledParagraph {
    #[serde(rename = "sup")]
    sup: Option<String>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "br")]
    br: Option<Br>,

    #[serde(rename = "content")]
    content: Option<TentacledContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledContent {
    #[serde(rename = "sup")]
    sup: Option<String>,

    #[serde(rename = "styleCode")]
    style_code: ContentStyleCode,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "linkHtml")]
    link_html: Option<Vec<LinkHtmlElement>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyParagraph {
    #[serde(rename = "br")]
    br: Option<Br>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "sub")]
    sub: Option<String>,

    #[serde(rename = "sup")]
    sup: Option<String>,

    #[serde(rename = "content")]
    content: Option<TentacledContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyTd {
    #[serde(rename = "paragraph")]
    paragraph: IndigoParagraph,

    #[serde(rename = "colspan")]
    colspan: String,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndigoParagraph {
    #[serde(rename = "br")]
    br: String,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "content")]
    content: Option<TitleContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyTable {
    #[serde(rename = "caption")]
    caption: Option<FluffyCaption>,

    #[serde(rename = "col")]
    col: Vec<Col>,

    #[serde(rename = "tbody")]
    tbody: FluffyTbody,

    #[serde(rename = "cellpadding")]
    cellpadding: Celling,

    #[serde(rename = "cellspacing")]
    cellspacing: Celling,

    #[serde(rename = "width")]
    width: Width,

    #[serde(rename = "thead")]
    thead: Option<Thead>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyTbody {
    #[serde(rename = "tr")]
    tr: Vec<FluffyTr>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyTr {
    #[serde(rename = "td")]
    td: MagentaTd,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledTd {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph7>,

    #[serde(rename = "align")]
    align: Option<Align>,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "rowspan")]
    rowspan: Option<String>,

    #[serde(rename = "colspan")]
    colspan: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndecentParagraph {
    #[serde(rename = "sup")]
    sup: Option<String>,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "br")]
    br: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyTd {
    #[serde(rename = "paragraph")]
    paragraph: Paragraph8,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "align")]
    align: Option<Align>,

    #[serde(rename = "colspan")]
    colspan: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Thead {
    #[serde(rename = "tr")]
    tr: Vec<TheadTr>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TheadTr {
    #[serde(rename = "th")]
    th: Vec<Th>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Th {
    #[serde(rename = "content")]
    content: ThContent,

    #[serde(rename = "align")]
    align: String,

    #[serde(rename = "styleCode")]
    style_code: String,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "br")]
    br: Option<String>,

    #[serde(rename = "colspan")]
    colspan: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ParagraphClass {
    #[serde(rename = "content")]
    content: TitleContent,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyComponent {
    // #[serde(rename = "observationMedia")]
    // observation_media: Option<ObservationMedia>,

    #[serde(rename = "section")]
    section: Option<TentacledSection>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ObservationMedia {
    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "value")]
    value: ObservationMediaValue,

    #[serde(rename = "ID")]
    id: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ObservationMediaValue {
    #[serde(rename = "reference")]
    reference: EffectiveTime,

    #[serde(rename = "mediaType")]
    media_type: String,

    #[serde(rename = "xsi:type")]
    xsi_type: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledSection {
    // #[serde(rename = "id")]
    // section_id: SetIdClass,

    #[serde(rename = "code")]
    code: Option<FormCode>,

    #[serde(rename = "title")]
    title: Option<String>,

    // #[serde(rename = "text")]
    // text: FluffyText,

    // #[serde(rename = "effectiveTime")]
    // effective_time: EffectiveTime,

    // #[serde(rename = "ID")]
    // id: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyText {
    #[serde(rename = "paragraph")]
    paragraph: Vec<Paragraph9>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct HilariousParagraph {
    #[serde(rename = "content")]
    content: IndecentContent,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "sub")]
    sub: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Excerpt {
    #[serde(rename = "highlight")]
    highlight: Highlight,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Highlight {
    #[serde(rename = "text")]
    text: HighlightText,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct HighlightText {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph10>,

    #[serde(rename = "list")]
    list: Option<BraggadociousList>,

    #[serde(rename = "table")]
    table: Option<TentacledTable>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndigoList {
    #[serde(rename = "item")]
    item: IndigoItem,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndigoItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "linkHtml")]
    link_html: LinkHtmlElement,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndecentList {
    #[serde(rename = "item")]
    item: FriskyItem,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndecentItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "linkHtml")]
    link_html: Option<LinkHtmlUnion>,

    #[serde(rename = "content")]
    content: Option<HilariousContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AmbitiousParagraph {
    #[serde(rename = "linkHtml")]
    link_html: Option<Vec<LinkHtmlElement>>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "content")]
    content: Option<TitleContent>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct CunningParagraph {
    #[serde(rename = "linkHtml")]
    link_html: LinkHtmlElement,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledTable {
    #[serde(rename = "col")]
    col: Vec<Col>,

    #[serde(rename = "tbody")]
    tbody: TentacledTbody,

    #[serde(rename = "cellpadding")]
    cellpadding: Celling,

    #[serde(rename = "cellspacing")]
    cellspacing: Celling,

    #[serde(rename = "width")]
    width: Width,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledTbody {
    #[serde(rename = "tr")]
    tr: Vec<TentacledTr>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledTr {
    #[serde(rename = "td")]
    td: FriskyTd,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndigoTd {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph12>,

    #[serde(rename = "align")]
    align: Option<Align>,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "list")]
    list: Option<HilariousList>,

    #[serde(rename = "colspan")]
    colspan: Option<String>,

    #[serde(rename = "rowspan")]
    rowspan: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct HilariousList {
    #[serde(rename = "item")]
    item: MischievousItem,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct HilariousItem {
    #[serde(rename = "caption")]
    caption: String,

    #[serde(rename = "content")]
    content: Option<TentacledContent>,

    #[serde(rename = "br")]
    br: Option<String>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MagentaParagraph {
    #[serde(rename = "content")]
    content: Option<TentacledContent>,

    #[serde(rename = "br")]
    br: Option<String>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndecentTd {
    #[serde(rename = "paragraph")]
    paragraph: ParagraphClass,

    #[serde(rename = "colspan")]
    colspan: String,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Subject {
    #[serde(rename = "manufacturedProduct")]
    pub manufactured_product: Option<ManufacturedProduct>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ManufacturedProduct {
    #[serde(rename = "manufacturedProduct")]
    pub manufactured_medicine: Option<ManufacturedMedicine>,

    #[serde(rename = "consumedIn")]
    pub consumed_in: Option<Vec<ConsumedIn>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ConsumedIn {
    #[serde(rename = "substanceAdministration")]
    pub substance_administration: Option<SubstanceAdministration>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct SubstanceAdministration {
    #[serde(rename = "routeCode")]
    pub route_code: FormCode,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ManufacturedMedicine {
    #[serde(rename = "code")]
    pub code: DefiningMaterialKindCode,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "formCode")]
    pub form_code: Option<FormCode>,

    #[serde(rename = "ingredient")]
    pub ingredient: Option<HashSet<Ingredient>>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AsContent {
    #[serde(rename = "quantity")]
    quantity: Option<AsContentQuantity>,

    // #[serde(rename = "containerPackagedProduct")]
    // container_packaged_product: ContainerPackagedProduct,

    // #[serde(rename = "subjectOf")]
    // subject_of: Vec<AsContentSubjectOf>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ContainerPackagedProduct {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "formCode")]
    form_code: FormCode,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AsContentQuantity {
    #[serde(rename = "numerator")]
    numerator: Option<Ator>,

    // #[serde(rename = "denominator")]
    // denominator: EffectiveTime,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Ator {
    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "unit")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AsContentSubjectOf {
    #[serde(rename = "characteristic")]
    characteristic: Option<PurpleCharacteristic>,

    // #[serde(rename = "marketingAct")]
    // marketing_act: Option<MarketingAct>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct PurpleCharacteristic {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "value")]
    value: FormCode,

    #[serde(rename = "classCode")]
    class_code: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MarketingAct {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "statusCode")]
    status_code: StatusCode,

    #[serde(rename = "effectiveTime")]
    effective_time: MarketingActEffectiveTime,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MarketingActEffectiveTime {
    #[serde(rename = "low")]
    low: EffectiveTime,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StatusCode {
    #[serde(rename = "code")]
    code: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AsEntityWithGeneric {
    #[serde(rename = "genericMedicine")]
    pub generic_medicine: Option<GenericMedicine>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct GenericMedicine {
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AsEquivalentEntity {
    // #[serde(rename = "code")]
    // code: DefiningMaterialKindCode,

    #[serde(rename = "definingMaterialKind")]
    defining_material_kind: DefiningMaterialKind,

    #[serde(rename = "classCode")]
    class_code: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Ingredient {
    #[serde(rename = "quantity")]
    pub quantity: Option<Quantity>,
    //
    // #[serde(rename = "ingredientSubstance")]
    // ingredient_substance: IngredientSubstance,
    //
    // #[serde(rename = "classCode")]
    // class_code: ClassCode,
}

impl Hash for Ingredient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.quantity {
            None => {
                None::<Ingredient>.hash(state);
            }
            Some(ref q) => {
                &q.denominator.unit.hash(state);
                &q.denominator.value.hash(state);
                &q.numerator.unit.hash(state);
                &q.numerator.value.hash(state);
            }
        }
    }
}

impl Eq for Ingredient {}
impl PartialEq for Ingredient {
    fn eq(&self, other: &Self) -> bool {
        match &self.quantity {
            None => {
                match &other.quantity {
                    None => {
                        true
                    }
                    Some(_) => {
                        false
                    }
                }
            }
            Some(ref q) => {
                match &other.quantity {
                    None => {
                        false
                    }
                    Some(q2) => {
                        if &q.numerator.value == &q2.numerator.value &&
                            &q.numerator.unit == &q2.numerator.unit &&
                            &q.denominator.value == &q2.denominator.value &&
                            &q.denominator.unit == &q2.denominator.unit{
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IngredientSubstance {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "activeMoiety")]
    active_moiety: Option<IngredientSubstanceActiveMoiety>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IngredientSubstanceActiveMoiety {
    #[serde(rename = "activeMoiety")]
    active_moiety: ActiveMoietyActiveMoiety,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ActiveMoietyActiveMoiety {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Quantity {
    #[serde(rename = "numerator")]
    pub numerator: Ator,
    //
    #[serde(rename = "denominator")]
    pub denominator: Ator,
}


#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ManufacturedProductSubjectOf {
    // #[serde(rename = "approval")]
    // approval: Option<Approval>,

    #[serde(rename = "marketingAct")]
    marketing_act: Option<MarketingAct>,

    #[serde(rename = "characteristic")]
    characteristic: Option<FluffyCharacteristic>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Approval {
    #[serde(rename = "id")]
    id: AssignedOrganizationId,

    #[serde(rename = "code")]
    code: FormCode,

    #[serde(rename = "author")]
    author: ApprovalAuthor,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ApprovalAuthor {
    #[serde(rename = "territorialAuthority")]
    territorial_authority: TerritorialAuthority,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TerritorialAuthority {
    #[serde(rename = "territory")]
    territory: DefiningMaterialKind,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FluffyCharacteristic {
    #[serde(rename = "code")]
    code: DefiningMaterialKindCode,

    #[serde(rename = "value")]
    value: CharacteristicValue,

    #[serde(rename = "classCode")]
    class_code: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct CharacteristicValue {
    #[serde(rename = "originalText")]
    original_text: Option<String>,

    #[serde(rename = "code")]
    code: Option<String>,

    #[serde(rename = "codeSystem")]
    code_system: Option<FormCodeCodeSystem>,

    #[serde(rename = "displayName")]
    display_name: Option<String>,

    #[serde(rename = "xsi:type")]
    xsi_type: String,

    #[serde(rename = "unit")]
    unit: Option<String>,

    #[serde(rename = "value")]
    value: Option<String>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TentacledText {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph13>,

    #[serde(rename = "list")]
    list: Option<List1>,

    #[serde(rename = "table")]
    table: Option<Vec<StickyTable>>,

    #[serde(rename = "renderMultiMedia")]
    render_multi_media: Option<RenderMultiMedia>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AmbitiousList {
    #[serde(rename = "item")]
    item: BraggadociousItem,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AmbitiousItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "content")]
    content: Option<AmbitiousContent>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct CunningList {
    #[serde(rename = "item")]
    item: Vec<CunningItem>,

    #[serde(rename = "listType")]
    list_type: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct CunningItem {
    #[serde(rename = "caption")]
    caption: String,

    #[serde(rename = "content")]
    content: Option<IndigoContent>,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FriskyParagraph {
    #[serde(rename = "sub")]
    sub: Option<Vec<String>>,

    #[serde(rename = "text")]
    text: Option<String>,

    #[serde(rename = "linkHtml")]
    link_html: Option<LinkHtmlUnion>,

    #[serde(rename = "content")]
    content: Option<CunningContent>,

    #[serde(rename = "br")]
    br: Option<Br>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MischievousParagraph {
    #[serde(rename = "content")]
    content: FluffyContent,

    #[serde(rename = "br")]
    br: Option<Vec<String>>,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct RenderMultiMedia {
    #[serde(rename = "ID")]
    id: Option<String>,

    #[serde(rename = "referencedObject")]
    referenced_object: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyTable {
    #[serde(rename = "caption")]
    caption: Option<String>,

    #[serde(rename = "col")]
    col: Vec<Col>,

    #[serde(rename = "tbody")]
    tbody: StickyTbody,

    #[serde(rename = "cellpadding")]
    cellpadding: Option<Celling>,

    #[serde(rename = "cellspacing")]
    cellspacing: Option<Celling>,

    #[serde(rename = "width")]
    width: Width,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyTbody {
    #[serde(rename = "tr")]
    tr: TrUnion,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct StickyTr {
    #[serde(rename = "td")]
    td: MischievousTd,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct HilariousTd {
    #[serde(rename = "paragraph")]
    paragraph: Option<Paragraph15>,

    #[serde(rename = "align")]
    align: Option<Align>,

    #[serde(rename = "styleCode")]
    style_code: Option<FluffyStyleCode>,

    #[serde(rename = "valign")]
    valign: Valign,

    #[serde(rename = "list")]
    list: Option<MagentaList>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MagentaList {
    #[serde(rename = "item")]
    item: Item1,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MagentaItem {
    #[serde(rename = "caption")]
    caption: CaptionEnum,

    #[serde(rename = "content")]
    content: Option<FluffyContent>,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct BraggadociousParagraph {
    #[serde(rename = "content")]
    content: Option<AmbitiousContent>,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "br")]
    br: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Paragraph1 {
    #[serde(rename = "content")]
    content: Option<PurpleContent>,

    #[serde(rename = "br")]
    br: Option<String>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AmbitiousTd {
    #[serde(rename = "paragraph")]
    paragraph: Paragraph2,

    #[serde(rename = "colspan")]
    colspan: String,

    #[serde(rename = "styleCode")]
    style_code: PurpleStyleCode,

    #[serde(rename = "valign")]
    valign: Valign,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Paragraph2 {
    #[serde(rename = "content")]
    content: TitleContent,

    #[serde(rename = "br")]
    br: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct IndigoTr {
    #[serde(rename = "td")]
    td: Vec<TrTdClass>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct TrTdClass {
    #[serde(rename = "list")]
    list: Option<FriskyList>,

    #[serde(rename = "styleCode")]
    style_code: String,

    #[serde(rename = "valign")]
    valign: Valign,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct FriskyList {
    #[serde(rename = "item")]
    item: Item2,

    #[serde(rename = "listType")]
    list_type: ListType,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct DocumentTitle {
    #[serde(rename = "br")]
    br: Option<Vec<String>>,

    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentUnion {
    FluffyComponent(FluffyComponent),

    PurpleComponentArray(Vec<PurpleComponent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousList {
    PurpleListArray(Vec<PurpleList>),

    TentacledList(TentacledList),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph3 {
    FluffyParagraph(FluffyParagraph),

    String(String),

    UnionArray(Vec<Paragraph4>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph4 {
    PurpleParagraph(PurpleParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyContent {
    PurpleContent(PurpleContent),

    PurpleContentArray(Vec<PurpleContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkHtmlUnion {
    LinkHtmlElement(LinkHtmlElement),

    LinkHtmlElementArray(Vec<LinkHtmlElement>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Br {
    String(String),

    StringArray(Vec<String>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoContent {
    FluffyContentArray(Vec<FluffyContent>),

    PurpleContent(PurpleContent),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableUnion {
    FluffyTable(FluffyTable),

    PurpleTableArray(Vec<PurpleTable>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleCaption {
    CaptionClass(CaptionClass),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningTd {
    FluffyTd(FluffyTd),

    PurpleTdArray(Vec<PurpleTd>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph5 {
    StickyParagraph(StickyParagraph),

    UnionArray(Vec<Paragraph6>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph6 {
    String(String),

    TentacledParagraph(TentacledParagraph),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyCaption {
    CaptionClass(CaptionClass),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaTd {
    StickyTd(StickyTd),

    TentacledTdArray(Vec<TentacledTd>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph7 {
    IndecentParagraphArray(Vec<IndecentParagraph>),

    String(String),

    TentacledParagraph(TentacledParagraph),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph8 {
    IndecentParagraph(IndecentParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThContent {
    TitleContent(TitleContent),

    TitleContentArray(Vec<TitleContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum TitleUnion {
    ParagraphClass(ParagraphClass),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph9 {
    HilariousParagraph(HilariousParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentContent {
    FluffyContent(FluffyContent),

    TitleContentArray(Vec<TitleContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousList {
    IndecentList(IndecentList),

    IndigoListArray(Vec<IndigoList>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyItem {
    IndecentItemArray(Vec<IndecentItem>),

    IndigoItem(IndigoItem),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousContent {
    PurpleContent(PurpleContent),

    TitleContentArray(Vec<TitleContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph10 {
    CunningParagraph(CunningParagraph),

    UnionArray(Vec<Paragraph11>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph11 {
    AmbitiousParagraph(AmbitiousParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyTd {
    IndecentTd(IndecentTd),

    IndigoTdArray(Vec<IndigoTd>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousItem {
    HilariousItem(HilariousItem),

    TentacledItemArray(Vec<TentacledItem>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph12 {
    MagentaParagraph(MagentaParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum List1 {
    AmbitiousListArray(Vec<AmbitiousList>),

    CunningList(CunningList),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousItem {
    AmbitiousItemArray(Vec<AmbitiousItem>),

    StickyItem(StickyItem),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousContent {
    FluffyContent(FluffyContent),

    FluffyContentArray(Vec<FluffyContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph13 {
    MischievousParagraph(MischievousParagraph),

    String(String),

    UnionArray(Vec<Paragraph14>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph14 {
    FriskyParagraph(FriskyParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningContent {
    TentacledContent(TentacledContent),

    TitleContentArray(Vec<TitleContent>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrUnion {
    IndigoTr(IndigoTr),

    StickyTrArray(Vec<StickyTr>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousTd {
    AmbitiousTd(AmbitiousTd),

    HilariousTdArray(Vec<HilariousTd>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item1 {
    FluffyItem(FluffyItem),

    MagentaItemArray(Vec<MagentaItem>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph15 {
    Paragraph1(Paragraph1),

    String(String),

    UnionArray(Vec<Paragraph16>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Paragraph16 {
    BraggadociousParagraph(BraggadociousParagraph),

    String(String),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item2 {
    FluffyItem(FluffyItem),

    FluffyItemArray(Vec<FluffyItem>),
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum FormCodeCodeSystem {
    #[serde(rename = "2.16.840.1.113883.3.26.1.1")]
    The216840111388332611,

    #[serde(rename = "2.16.840.1.113883.6.1")]
    The216840111388361,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum PurpleCodeSystem {
    #[serde(rename = "2.16.840.1.113883.1.11.19255")]
    The216840111388311119255,

    #[serde(rename = "2.16.840.1.113883.3.26.1.1")]
    The216840111388332611,

    #[serde(rename = "2.16.840.1.113883.4.9")]
    The216840111388349,

    #[serde(rename = "2.16.840.1.113883.5.28")]
    The2168401113883528,

    #[serde(rename = "2.16.840.1.113883.6.69")]
    The2168401113883669,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum CaptionEnum {
    #[serde(rename = "•")]
    Empty,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum ListType {
    #[serde(rename = "unordered")]
    Unordered,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum ContentStyleCode {
    #[serde(rename = "bold")]
    Bold,

    #[serde(rename = "italics")]
    Italics,

    #[serde(rename = "underline")]
    Underline,

    #[serde(rename = "xmChange")]
    XmChange,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum Celling {
    #[serde(rename = "0pt")]
    The0Pt,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum Align {
    #[serde(rename = "center")]
    Center,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum PurpleStyleCode {
    #[serde(rename = "Botrule Lrule ")]
    BotruleLrule,

    #[serde(rename = "Lrule Botrule ")]
    LruleBotrule,

    #[serde(rename = "Rrule Botrule ")]
    RruleBotrule,

    #[serde(rename = "Rrule Botrule Lrule ")]
    RruleBotruleLrule,

    #[serde(rename = "Rrule Botrule Lrule Toprule ")]
    RruleBotruleLruleToprule,

    #[serde(rename = "Rrule Botrule Toprule ")]
    RruleBotruleToprule,

    #[serde(rename = "Rrule Lrule Botrule ")]
    RruleLruleBotrule,

    #[serde(rename = "Rrule Lrule Toprule Botrule ")]
    RruleLruleTopruleBotrule,

    #[serde(rename = "Rrule Toprule Botrule ")]
    RruleTopruleBotrule,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum Valign {
    #[serde(rename = "top")]
    Top,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum Width {
    #[serde(rename = "100%")]
    The100,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum ClassCode {
    #[serde(rename = "ACTIB")]
    Actib,

    #[serde(rename = "IACT")]
    Iact,
}

#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum FluffyStyleCode {
    #[serde(rename = "Botrule ")]
    Botrule,

    #[serde(rename = "Rrule Botrule ")]
    RruleBotrule,

    #[serde(rename = "Rrule Botrule Lrule ")]
    RruleBotruleLrule,

    #[serde(rename = "Rrule Lrule Botrule ")]
    RruleLruleBotrule,

    #[serde(rename = "Toprule ")]
    Toprule,
}
