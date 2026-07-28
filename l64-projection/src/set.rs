use crate::{
    AtlasView, CertificationView, ContextAnalysis, ProjectionError, ProjectionSource, ReplayView,
    ReportView, ResearchView,
};
use l64_native::{ContextId, Graph, Route};
use std::fmt::Write as _;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSet {
    pub source: ProjectionSource,
    pub atlas: AtlasView,
    pub certification: CertificationView,
    pub replay: ReplayView,
    pub report: ReportView,
    pub research: ResearchView,
}

impl ProjectionSet {
    pub fn derive(
        graph: &Graph,
        context: ContextId,
        research_limit: usize,
    ) -> Result<Self, ProjectionError> {
        let analysis = ContextAnalysis::derive(graph, context)?;
        let atlas = AtlasView::derive_analyzed(graph, &analysis)?;
        Ok(Self {
            source: analysis.source.clone(),
            certification: CertificationView::from_atlas(&atlas),
            replay: ReplayView::derive_analyzed(graph, &analysis)?,
            report: ReportView::derive_analyzed(graph, &analysis)?,
            research: ResearchView::derive_analyzed(graph, &analysis, research_limit)?,
            atlas,
        })
    }

    pub fn verify(&self, graph: &Graph) -> Result<(), ProjectionError> {
        self.source.verify(graph)?;
        let rebuilt = Self::derive(graph, self.source.context, self.research.limit)?;
        if &rebuilt == self {
            Ok(())
        } else {
            Err(ProjectionError::ProjectionMismatch)
        }
    }

    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE PROJECTION v{}", self.source.version);
        let _ = writeln!(out, "context={}", self.source.context);
        let _ = writeln!(out, "symbol={}", self.source.symbol);
        let _ = writeln!(out, "nodes={}", self.source.node_count);
        let _ = writeln!(out, "contexts={}", self.source.context_count);
        let _ = writeln!(out, "journal={}", self.source.journal_len);
        let _ = writeln!(out, "atlas_candidates={}", self.atlas.candidates.len());
        let _ = writeln!(
            out,
            "certification_burdens={}",
            self.certification.burdens.len()
        );
        let _ = writeln!(out, "replay_steps={}", self.replay.steps.len());
        let _ = writeln!(out, "closed={}", self.report.closed);
        let _ = writeln!(out, "open={}", self.report.open);
        let _ = writeln!(out, "invalid={}", self.report.invalid);
        let _ = writeln!(
            out,
            "research_candidates={}/{}",
            self.research.candidates.len(),
            self.research.total_candidates
        );
        for candidate in &self.research.candidates {
            let _ = writeln!(
                out,
                "research route={} opcode={:?} closure={:?} affected={} dependents={}",
                format_route(&candidate.source.route),
                candidate.opcode,
                candidate.closure,
                candidate.affected_nodes,
                candidate.direct_dependents
            );
        }
        out
    }
}
fn format_route(route: &Route) -> String {
    let mut out = format!("{:016x}", route.domain().0);
    for word in route.tail() {
        let _ = write!(out, "/{:016x}", word.0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_native::LocusWord;

    #[test]
    fn route_format_is_numeric_and_stable() {
        let route = Route::root(LocusWord(0x12)).composed(LocusWord(0x34));
        assert_eq!(format_route(&route), "0000000000000012/0000000000000034");
    }
}
