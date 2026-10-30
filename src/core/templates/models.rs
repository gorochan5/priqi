use crate::core::widgets::import_modal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleTemplateKind {
    ProdutoAcabado,
    MateriaPrima,
    LocalEstoque,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub sample_kind: SampleTemplateKind,
    pub zpl: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    SelectTemplate(String),
    DeleteTemplate(String),
    DuplicateTemplate,
    AddTemplate,
    StartRename(String),
    RenameInputChanged(String),
    SaveRename,
    CancelRename,
    ImportModal(import_modal::Message),
}

#[derive(Debug)]
pub struct State {
    pub templates: Vec<Template>,
    pub selected_id: Option<String>,
    pub editing_id: Option<String>,
    pub editing_name: String,
    pub template_counter: usize,
    pub import_modal: import_modal::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            templates: vec![
                Template {
                    id: "produto_acabado".to_string(),
                    name: "Etiqueta de Produto Acabado".to_string(),
                    sample_kind: SampleTemplateKind::ProdutoAcabado,
                    zpl: "^XA\n^FO50,40^A0N,28,28^FD[EMP_ERAZAO]^FS\n^FO50,80^A0N,32,32^FD[PRO_CODPRO]^FS\n^FO50,120^A0N,22,22^FD[PRO_DESCR]^FS\n^FO450,40^BY2^BCN,140,N,N,N,U^FD[COD_BARRAS]^FS\n^FO50,170^GB600,1,2^FS\n^FO50,190^A0N,18,18^FDFab: [ETI_COMP04]^FS\n^FO250,190^A0N,18,18^FDLote: [ETI_LOTEFG]^FS\n^FO450,190^A0N,18,18^FDP. Bruto: [ETI_PESOBR]^FS\n^FO50,225^A0N,18,18^FDVal: [ETI_COMP05]^FS\n^FO250,225^A0N,18,18^FDQtd: [ETI_QTDENV]^FS\n^FO450,225^A0N,18,18^FDP. Liq: [ETI_PESOLQ]^FS\n^XZ".to_string(),
                },
                Template {
                    id: "materia_prima".to_string(),
                    name: "Etiqueta de Matéria Prima".to_string(),
                    sample_kind: SampleTemplateKind::MateriaPrima,
                    zpl: "^XA\n^FO50,40^A0N,24,24^FDCliente: [EMP_ERAZAO]^FS\n^FO50,80^A0N,44,44^FD[PRO_CODPRO]^FS\n^FO50,135^A0N,22,22^FD[PRO_DESCR]^FS\n^FO450,40^BY3^BCN,150,N,N,N,U^FD[COD_BARRAS]^FS\n^FO50,185^GB600,1,2^FS\n^FO50,205^A0N,20,20^FDFab: [ETI_COMP04]^FS\n^FO250,205^A0N,20,20^FDLote: [ETI_LOTEFG]^FS\n^FO450,205^A0N,20,20^FDP. Bruto: [ETI_PESOBR]^FS\n^XZ".to_string(),
                },
                Template {
                    id: "local_estoque".to_string(),
                    name: "Etiqueta de Local de Estoque".to_string(),
                    sample_kind: SampleTemplateKind::LocalEstoque,
                    zpl: "^XA\n^FO80,60^BY3^BCN,120,Y,N,N^FD[ETI_LOCEST]^FS\n^XZ".to_string(),
                },
            ],
            selected_id: Some("produto_acabado".to_string()),
            editing_id: None,
            editing_name: String::new(),
            template_counter: 3,
            import_modal: import_modal::State::default(),
        }
    }
}
