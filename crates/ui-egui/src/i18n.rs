//! Interface translations. Command ids, document text and file names remain stable.
//! Untranslated labels fall back to English so coverage can grow incrementally.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
    /// BCP-47 lower-case code: the variant name alone would serialize as `ptbr`.
    #[serde(rename = "pt-br")]
    PtBr,
}

impl Language {
    pub const ALL: [Self; 3] = [Self::En, Self::Ja, Self::PtBr];

    /// The persisted / `ui.set` code (`en`, `ja`, `pt-br`).
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ja => "ja",
            Self::PtBr => "pt-br",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
            Self::PtBr => "Português (Brasil)",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            "pt-br" => Some(Self::PtBr),
            _ => None,
        }
    }

    pub fn tr(self, text: &str) -> &str {
        let table = match self {
            Self::En => return text,
            Self::Ja => JAPANESE,
            Self::PtBr => PORTUGUESE,
        };
        if let Some((_, translated)) = table.iter().find(|(english, _)| *english == text) {
            return translated;
        }
        text
    }
}

const JAPANESE: &[(&str, &str)] = &[
    ("Menu", "メニュー"),
    ("File", "ファイル"),
    ("Edit", "編集"),
    ("Pages", "ページ"),
    ("View", "表示"),
    ("Help", "ヘルプ"),
    ("Preferences", "環境設定"),
    ("Preferences…", "環境設定…"),
    ("Interface language", "表示言語"),
    ("Identity", "個人情報"),
    ("Name on new comments", "新しい注釈の作成者名"),
    ("Open…", "開く…"),
    ("New blank PDF", "空白の PDF を作成"),
    ("Create PDF from file…", "ファイルから PDF を作成…"),
    ("Create PDF from images…", "画像から PDF を作成…"),
    ("Create PDF from clipboard", "クリップボードから PDF を作成"),
    ("Combine files…", "ファイルを結合…"),
    ("Save", "保存"),
    ("Save as…", "別名で保存…"),
    ("Close file", "ファイルを閉じる"),
    ("Close all", "すべて閉じる"),
    ("Revert", "保存済みの状態に戻す"),
    ("Print…", "印刷…"),
    ("Document properties…", "文書のプロパティ…"),
    ("Undo", "取り消し"),
    ("Redo", "やり直し"),
    ("Find…", "検索…"),
    ("Advanced search…", "高度な検索…"),
    ("Copy pages", "ページをコピー"),
    ("Cut pages", "ページを切り取り"),
    ("Paste pages", "ページを貼り付け"),
    ("Fit visible", "表示範囲に合わせる"),
    ("Marquee zoom", "範囲指定ズーム"),
    ("Take a snapshot", "スナップショットを作成"),
    ("Full screen mode", "全画面表示"),
    ("Read mode", "閲覧モード"),
    ("Switch light / dark theme", "明るい／暗いテーマを切り替え"),
    ("Comments panel", "コメントパネル"),
    ("Form fields panel", "フォームフィールドパネル"),
    ("Clear form", "フォームをクリア"),
    ("Find tools and commands…", "ツールとコマンドを検索…"),
    ("Zoom", "ズーム"),
    ("Actual size", "実際のサイズ"),
    ("Zoom to page level", "ページ全体を表示"),
    ("Fit to width", "幅に合わせる"),
    ("Display theme", "表示テーマ"),
    ("Side panels", "サイドパネル"),
    ("Enable Acrobat JavaScript", "Acrobat JavaScript を有効にする"),
    ("OK", "OK"),
];

/// Brazilian Portuguese: the same keys as `JAPANESE`, translated for pt-BR.
const PORTUGUESE: &[(&str, &str)] = &[
    ("Menu", "Menu"),
    ("File", "Arquivo"),
    ("Edit", "Editar"),
    ("Pages", "Páginas"),
    ("View", "Exibir"),
    ("Help", "Ajuda"),
    ("Preferences", "Preferências"),
    ("Preferences…", "Preferências…"),
    ("Interface language", "Idioma da interface"),
    ("Identity", "Identidade"),
    ("Name on new comments", "Nome nos novos comentários"),
    ("Open…", "Abrir…"),
    ("New blank PDF", "Novo PDF em branco"),
    ("Create PDF from file…", "Criar PDF a partir de arquivo…"),
    ("Create PDF from images…", "Criar PDF a partir de imagens…"),
    ("Create PDF from clipboard", "Criar PDF a partir da área de transferência"),
    ("Combine files…", "Combinar arquivos…"),
    ("Save", "Salvar"),
    ("Save as…", "Salvar como…"),
    ("Close file", "Fechar arquivo"),
    ("Close all", "Fechar tudo"),
    ("Revert", "Reverter"),
    ("Print…", "Imprimir…"),
    ("Document properties…", "Propriedades do documento…"),
    ("Undo", "Desfazer"),
    ("Redo", "Refazer"),
    ("Find…", "Localizar…"),
    ("Advanced search…", "Pesquisa avançada…"),
    ("Copy pages", "Copiar páginas"),
    ("Cut pages", "Recortar páginas"),
    ("Paste pages", "Colar páginas"),
    ("Fit visible", "Ajustar ao visível"),
    ("Marquee zoom", "Zoom por seleção"),
    ("Take a snapshot", "Capturar instantâneo"),
    ("Full screen mode", "Modo de tela cheia"),
    ("Read mode", "Modo de leitura"),
    ("Switch light / dark theme", "Alternar tema claro / escuro"),
    ("Comments panel", "Painel de comentários"),
    ("Form fields panel", "Painel de campos de formulário"),
    ("Clear form", "Limpar formulário"),
    ("Find tools and commands…", "Localizar ferramentas e comandos…"),
    ("Zoom", "Zoom"),
    ("Actual size", "Tamanho real"),
    ("Zoom to page level", "Ajustar à página"),
    ("Fit to width", "Ajustar à largura"),
    ("Display theme", "Tema de exibição"),
    ("Side panels", "Painéis laterais"),
    ("Enable Acrobat JavaScript", "Ativar o Acrobat JavaScript"),
    ("OK", "OK"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_are_unique_and_preserve_unknown_text() {
        for (i, (en, ja)) in JAPANESE.iter().enumerate() {
            assert!(!ja.is_empty());
            assert!(JAPANESE.iter().take(i).all(|(other, _)| en != other));
            assert_eq!(Language::En.tr(en), *en);
        }
        assert_eq!(Language::Ja.tr("File"), "ファイル");
        assert_eq!(Language::Ja.tr("日本語の文書.pdf"), "日本語の文書.pdf");
        assert_eq!(Language::parse("xx"), None);
    }

    #[test]
    fn portuguese_translations_are_unique_and_cover_the_same_keys() {
        assert_eq!(PORTUGUESE.len(), JAPANESE.len());
        for (i, (en, pt)) in PORTUGUESE.iter().enumerate() {
            assert!(!pt.is_empty());
            assert!(PORTUGUESE.iter().take(i).all(|(other, _)| en != other));
            assert_eq!(Language::En.tr(en), *en);
        }
        // Same keys, same order, no entry added or dropped against the Japanese table.
        for ((en, ja), (pt_en, pt)) in JAPANESE.iter().zip(PORTUGUESE.iter()) {
            assert_eq!(en, pt_en);
            // Ellipsis style (`…`) is kept exactly as the English source uses it.
            assert_eq!(ja.contains('…'), pt.contains('…'));
        }
        assert_eq!(Language::PtBr.tr("File"), "Arquivo");
        assert_eq!(Language::PtBr.tr("Interface language"), "Idioma da interface");
        assert_eq!(Language::PtBr.tr("Save as…"), "Salvar como…");
        assert_eq!(Language::PtBr.tr("Arquivo do usuário.pdf"), "Arquivo do usuário.pdf");
    }

    #[test]
    fn language_codes_parse_name_and_persist_as_expected() {
        assert_eq!(Language::PtBr.code(), "pt-br");
        assert_eq!(Language::En.code(), "en");
        assert_eq!(Language::Ja.code(), "ja");
        assert_eq!(Language::PtBr.name(), "Português (Brasil)");
        assert_eq!(Language::parse("pt-br"), Some(Language::PtBr));
        assert_eq!(Language::ALL, [Language::En, Language::Ja, Language::PtBr]);
        for language in Language::ALL {
            assert_eq!(Language::parse(language.code()), Some(language));
            assert_eq!(serde_json::to_string(&language).unwrap().trim_matches('"'), language.code());
        }
        assert_eq!(serde_json::from_str::<Language>(r#""pt-br""#).unwrap(), Language::PtBr);
        assert!(serde_json::from_str::<Language>(r#""pt""#).is_err());
    }

    #[test]
    fn language_persists_and_invalid_input_keeps_current_language() {
        let mut app = crate::PrintCraftApp::default();
        app.set_option("language", "ja").unwrap();
        assert_eq!(app.language, Language::Ja);
        assert!(app.set_option("language", "xx").is_err());
        assert_eq!(app.language, Language::Ja);
        let mut restored = crate::PrintCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::Ja);
        restored.restore(r#"{"language":"xx"}"#);
        assert_eq!(restored.language, Language::Ja);
        let mut legacy = crate::PrintCraftApp::default();
        legacy.restore("{}");
        assert_eq!(legacy.language, Language::En);
    }

    #[test]
    fn portuguese_is_selected_persists_and_falls_back_on_unknown_code() {
        let mut app = crate::PrintCraftApp::default();
        app.set_option("language", "pt-br").unwrap();
        assert_eq!(app.language, Language::PtBr);
        assert!(app.set_option("language", "pt").is_err());
        assert_eq!(app.language, Language::PtBr);
        let mut restored = crate::PrintCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::PtBr);
        restored.restore(r#"{"language":"pt-br"}"#);
        assert_eq!(restored.language, Language::PtBr);
    }
}
